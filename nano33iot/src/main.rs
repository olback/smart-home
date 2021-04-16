#![no_std]
#![no_main]
#![feature(default_alloc_error_handler, panic_info_message, asm)]

extern crate alloc;

use {
    alloc_cortex_m::CortexMHeap,
    arduino_nano33iot::{
        self as hal,
        clock::GenericClockController,
        delay::Delay,
        entry,
        pac::{CorePeripherals, Peripherals},
        prelude::*,
    },
    config::CONFIG,
    core::time::Duration,
    dht_sensor::{dht22, DhtReading},
    display::{Display, XPos, YPos},
    embedded_graphics::fonts::{Font12x16, Font6x12, Font6x8},
    measurement::Measurement,
    wifi::WiFi,
};

mod config;
mod display;
mod measurement;
#[cfg(feature = "usb_logging")]
mod usb;
mod util;
mod wifi;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

static mut DELAY: Option<Delay> = None;

#[entry]
fn main() -> ! {
    // Init heap
    let heap_start = cortex_m_rt::heap_start() as usize;
    unsafe { ALLOCATOR.init(heap_start, 1024) };

    // Init IO
    let mut core = CorePeripherals::take().unwrap();
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    unsafe { DELAY = Some(Delay::new(core.SYST, &mut clocks)) }
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut led = pins.led_sck.into_open_drain_output(&mut pins.port);
    let mut sensor_outside = pins.d2.into_readable_open_drain_output(&mut pins.port);
    let mut sensor_inside = pins.d3.into_readable_open_drain_output(&mut pins.port);

    drop(led.set_high());

    // Init display
    let mut disp = Display::new(
        &mut clocks,
        400_000.hz(),
        peripherals.SERCOM4,
        &mut peripherals.PM,
        pins.sda,
        pins.scl,
        &mut pins.port,
    );

    disp.write(
        concat!("Data point ", env!("CARGO_PKG_VERSION")),
        Font6x12,
        XPos::Center(0),
        YPos::Center(0),
        true,
    );
    delay_ms!(1000u16);

    disp.clear(false);
    disp.write("USB Init", Font6x12, XPos::Left(0), YPos::Top(0), false);
    disp.write(
        "WiFi Connect",
        Font6x12,
        XPos::Left(0),
        YPos::Top(12),
        false,
    );
    disp.write(
        "Trigger read",
        Font6x12,
        XPos::Left(0),
        YPos::Top(24),
        false,
    );

    #[cfg(feature = "usb_logging")]
    {
        usb::init(
            peripherals.USB,
            &mut clocks,
            &mut peripherals.PM,
            pins.usb_dm,
            pins.usb_dp,
            &mut core.NVIC,
        );
        usb::init_logger();
        disp.write("done", Font6x12, XPos::Right(0), YPos::Top(0), true);
        log::info!("USB Init done");
    }
    #[cfg(not(feature = "usb_logging"))]
    disp.write("skipped", Font6x12, XPos::Right(0), YPos::Top(0), true);

    // Configure WiFi
    let mut wifi = WiFi::new(
        CONFIG.wifi.into_nina_config(),
        &mut clocks,
        &mut peripherals.PM,
        peripherals.SERCOM2,
        pins.nina_miso,
        pins.nina_mosi,
        pins.nina_sck,
        pins.nina_ack,
        pins.nina_resetn,
        pins.nina_cs,
        |duration| delay_us!(duration.as_micros() as u32),
        &mut pins.port,
    )
    .unwrap();
    while let Err(_) = wifi.configure(Some(Duration::from_secs(10))) {
        disp.write("err", Font6x12, XPos::Right(0), YPos::Top(12), true);
    }
    disp.write("done", Font6x12, XPos::Right(0), YPos::Top(12), true);
    log::info!("WiFi Connected");

    // First read
    let _ = dht22::Reading::read(delay!(), &mut sensor_outside);
    let _ = dht22::Reading::read(delay!(), &mut sensor_inside);
    delay_ms!(2000u16);
    disp.write("done", Font6x12, XPos::Right(0), YPos::Top(24), true);
    log::info!("First read done");

    // TODO: Use a interrupt based timer to start measurements

    drop(led.set_low());

    // Main loop

    loop {
        drop(led.set_high());

        // Read sensors
        let result_inside = dht22::Reading::read(delay!(), &mut sensor_inside);
        let result_outside = dht22::Reading::read(delay!(), &mut sensor_outside);

        // Write results to display
        disp.clear(false);
        disp.write("In", Font12x16, XPos::Left(0), YPos::Top(8), false);
        disp.write(
            &result_inside
                .as_ref()
                .map(|v| alloc::format!("{}°C", util::round(v.temperature)))
                .unwrap_or("Error".into()),
            Font12x16,
            XPos::Right(0),
            YPos::Top(8),
            false,
        );
        log::info!("{:?}", result_inside);
        disp.write("Out", Font12x16, XPos::Left(0), YPos::Bottom(8), false);
        disp.write(
            &result_outside
                .as_ref()
                .map(|v| alloc::format!("{}°C", util::round(v.temperature)))
                .unwrap_or("Error".into()),
            Font12x16,
            XPos::Right(0),
            YPos::Bottom(8),
            true,
        );
        log::info!("{:?}", result_outside);

        match result_outside
            .map(|raw| Measurement::new("outside", "", raw).to_http_req(CONFIG.server.endpoint))
        {
            Ok(value) => match wifi.http_post(CONFIG.server.host, CONFIG.server.port, &value, 2) {
                Ok(_) => {}
                Err(err) => {
                    disp.write(
                        "WiFi/Server error",
                        Font6x8,
                        XPos::Center(0),
                        YPos::Center(0),
                        true,
                    );
                    log::error!("{}", err)
                }
            },
            Err(err) => log::error!("{:?}", err),
        };

        match result_inside
            .map(|raw| Measurement::new("inside", "", raw).to_http_req(CONFIG.server.endpoint))
        {
            Ok(value) => match wifi.http_post(CONFIG.server.host, CONFIG.server.port, &value, 2) {
                Ok(_) => {}
                Err(err) => {
                    disp.write(
                        "WiFi/Server error",
                        Font6x8,
                        XPos::Center(0),
                        YPos::Center(0),
                        true,
                    );
                    log::error!("{}", err);
                }
            },
            Err(err) => log::error!("{:?}", err),
        };

        drop(led.set_low());

        delay_ms!(2000u16);
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    // log::error!("{}", info);
    let location = info.location();
    log::error!("== begin panic ==");
    log::error!(
        "Location: {}#{}:{}",
        location.map(|l| l.file()).unwrap_or("<unknown>"),
        location.map(|l| l.line()).unwrap_or(0),
        location.map(|l| l.column()).unwrap_or(0)
    );
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        log::error!("Cause: {:?}", s);
    } else {
        // log::error!("Payload: <unknown>");
        match info.message() {
            Some(m) => log::error!("Cause: {}", m),
            None => log::error!("Cause: unknown"),
        }
    }
    log::error!("== end panic ==");
    loop {}
}
