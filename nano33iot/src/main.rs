#![no_std]
#![no_main]
#![feature(default_alloc_error_handler, panic_info_message)]

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
    dht_sensor::{dht22, DhtReading},
    display::{Display, XPos, YPos},
    embedded_graphics::fonts::{Font12x16, Font6x12},
};

mod config;
mod display;
mod measurement;
mod usb;
mod util;
mod wifi;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

static mut DELAY: Option<Delay> = None;
static mut OUTSIDE: Option<dht22::Reading> = None;
static mut INSIDE: Option<dht22::Reading> = None;

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
        "Create client",
        Font6x12,
        XPos::Left(0),
        YPos::Top(24),
        false,
    );
    disp.write("Trigger read", Font6x12, XPos::Left(0), YPos::Top(36), true);

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

    // disp.clear();
    let nina_spi = wifi::nina_spi_master(
        &mut clocks,
        &mut peripherals.PM,
        peripherals.SERCOM2,
        pins.nina_miso,
        pins.nina_mosi,
        pins.nina_sck,
        &mut pins.port,
    );

    log::info!("[WiFi NINA] SPI Master configured");

    let nina_spi_transport = wifi_nina::transport::SpiTransport::start(
        nina_spi,
        pins.nina_ack.into_floating_input(&mut pins.port),
        pins.nina_resetn.into_open_drain_output(&mut pins.port),
        pins.nina_cs.into_open_drain_output(&mut pins.port),
        |duration| delay_us!(duration.as_micros() as u32),
    );

    let mut nina_wifi = loop {
        match nina_spi_transport {
            Ok(nst) => {
                log::info!("[WiFi NINA] SPI Transport Configured");
                break wifi_nina::Wifi::new(nst);
            }
            Err(ref e) => log::error!("[WiFi NINA] SPI Transport failed {:?}", e),
        }
    };

    loop {
        // This has an internal await_connection_state
        match nina_wifi.configure(
            config::CONFIG.wifi.into_nina_config(),
            Some(core::time::Duration::from_secs(10)),
        ) {
            Ok(_) => {
                log::info!("[WiFi NINA] Connected to WiFi");
                break;
            }
            Err(ref e) => log::error!("[WiFi NINA] Connection failed {:?}", e),
        }
    }
    disp.write("done", Font6x12, XPos::Right(0), YPos::Top(12), true);

    let mut client = loop {
        match nina_wifi.new_client() {
            Ok(c) => {
                log::info!("[WiFi NINA] Client OK");
                break c;
            }
            Err(ref e) => log::error!("[WiFi NINA] Failed to get new client {:?}", e),
        }
    };
    disp.write("done", Font6x12, XPos::Right(0), YPos::Top(24), true);

    let _ = dht22::Reading::read(delay!(), &mut sensor_outside);
    let _ = dht22::Reading::read(delay!(), &mut sensor_inside);
    delay_ms!(2000u16);
    disp.write("done", Font6x12, XPos::Right(0), YPos::Top(36), true);

    drop(led.set_low());

    loop {
        drop(led.set_high());

        unsafe {
            OUTSIDE = dht22::Reading::read(delay!(), &mut sensor_outside).ok();
            INSIDE = dht22::Reading::read(delay!(), &mut sensor_inside).ok();
        }

        disp.clear(false);
        disp.write("In", Font12x16, XPos::Left(0), YPos::Top(8), false);
        disp.write(
            unsafe {
                &INSIDE
                    .map(|v| alloc::format!("{}°C", util::round(v.temperature)))
                    .unwrap_or("Err".into())
            },
            Font12x16,
            XPos::Right(0),
            YPos::Top(8),
            false,
        );
        disp.write("Out", Font12x16, XPos::Left(0), YPos::Bottom(8), false);
        disp.write(
            unsafe {
                &OUTSIDE
                    .map(|v| alloc::format!("{}°C", util::round(v.temperature)))
                    .unwrap_or("Err".into())
            },
            Font12x16,
            XPos::Right(0),
            YPos::Bottom(8),
            true,
        );

        drop(led.set_low());

        delay_ms!(2000u16);

        /*drop(led.set_high());
        match client.connect_ipv4(
            &mut nina_wifi,
            config::CONFIG.server.host,
            config::CONFIG.server.port,
            wifi_nina::types::ProtocolMode::Tcp,
        ) {
            Ok(_) => {
                log::info!("[WiFi NINA] Connected to server");
                // break;
            }
            Err(e) => {
                log::error!("Error connecting to server {:?}", e);
                delay_ms!(100u8);
                continue;
            }
        }

        let result_outside = dht22::Reading::read(delay!(), &mut sensor_outside);
        let result_inside = dht22::Reading::read(delay!(), &mut sensor_inside);
        log::info!("Outside: {:#?}", result_outside);
        delay_ms!(10u8);
        log::info!("Inside: {:#?}", result_inside);

        if result_outside.is_ok() {
            let mreq = measurement::Measurement::new("outside", "", result_outside.unwrap())
                .to_http_req("/api/temp-hum");
            match client.send_all(&mut nina_wifi, mreq.as_bytes()) {
                Ok(_) => log::info!("data sent ok"),
                Err(e) => log::error!("failed to send data: {:?}", e),
            };
        }

        drop(led.set_low());

        delay_ms!(2000u16);*/
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
