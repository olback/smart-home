#![no_std]
#![no_main]
#![feature(default_alloc_error_handler, panic_info_message, asm)]

extern crate alloc;

use {
    alloc_cortex_m::CortexMHeap,
    arduino_nano33iot::{
        self as hal,
        clock::{ClockGenId, ClockSource, GenericClockController},
        delay::Delay,
        entry,
        pac::{CorePeripherals, Peripherals, RTC},
        prelude::*,
        rtc,
        timer::TimerCounter5,
    },
    config::CONFIG,
    core::time::Duration,
    dht_sensor::{dht22, DhtReading},
    display::{Display, XPos, YPos},
    embedded_graphics::fonts::{Font12x16, Font6x12},
    measurement::Measurement,
    wifi::WiFi,
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
    disp.write("Timer setup", Font6x12, XPos::Left(0), YPos::Top(36), true);

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

    // Configure WiFi
    let mut wifi = WiFi::new(
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
    // while let Err(_) = wifi.connect_wifi(
    //     CONFIG.wifi.into_nina_config(),
    //     Some(Duration::from_secs(10)),
    // ) {
    //     disp.write("err", Font6x12, XPos::Right(0), YPos::Top(12), true);
    // }
    disp.write("done", Font6x12, XPos::Right(0), YPos::Top(12), true);
    log::info!("WiFi Connected");

    // First read
    let _ = dht22::Reading::read(delay!(), &mut sensor_outside);
    let _ = dht22::Reading::read(delay!(), &mut sensor_inside);
    delay_ms!(2000u16);
    disp.write("done", Font6x12, XPos::Right(0), YPos::Top(24), true);
    log::info!("First read done");

    // TODO: Use RTC instead?
    // let gclk1 = clocks.gclk1();
    // let tc45 = &clocks.tc4_tc5(&gclk1).unwrap();
    // let mut tim5 = TimerCounter5::tc5_(tc45, peripherals.TC5, &mut peripherals.PM);
    // tim5.start(1.s()); // Panics?
    // tim5.enable_interrupt();
    // unsafe {
    //     core.NVIC.set_priority(interrupt::TC5, 2);
    //     hal::pac::NVIC::unmask(interrupt::TC5);
    // }

    // hal::pac::NVIC::unpend(arduino_nano33iot::pac::interrupt::TC5);
    // unsafe { hal::pac::NVIC::unmask(arduino_nano33iot::pac::interrupt::TC5) };
    //

    // let timer_clock = clocks
    //     .configure_gclk_divider_and_source(ClockGenId::GCLK1, 1, ClockSource::OSC32K, false)
    //     .unwrap();
    // let timer_clock = clocks.gclk0();
    // let rtc_clock = clocks.rtc(&timer_clock).unwrap();
    // let mut timer = rtc::Rtc::count32_mode(peripherals.RTC, rtc_clock.freq(), &mut peripherals.PM);
    // timer.reset_and_compute_prescaler(1.hz());
    // timer.enable_interrupt();
    // timer.start(2.hz());
    // unsafe {
    //     core.NVIC.set_priority(interrupt::RTC, 2);
    //     hal::pac::NVIC::unmask(interrupt::RTC);
    // }
    disp.write("done", Font6x12, XPos::Right(0), YPos::Top(36), true);
    log::info!("Timer setup done");

    drop(led.set_low());

    // Main loop

    loop {
        drop(led.set_high());

        // Read sensors
        let result_outside = dht22::Reading::read(delay!(), &mut sensor_outside);
        let result_inside = dht22::Reading::read(delay!(), &mut sensor_inside);

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

        // Send results to server
        // match wifi.connect_wifi(
        //     CONFIG.wifi.into_nina_config(),
        //     Some(Duration::from_secs(10)),
        // ) {
        //     Ok(_) => {
        //         match result_outside.map(|raw| {
        //             Measurement::new("outside", "", raw).to_http_req(CONFIG.server.endpoint)
        //         }) {
        //             Ok(value) => {
        //                 match wifi.http_post(CONFIG.server.host, CONFIG.server.port, &value) {
        //                     Ok(_) => {}
        //                     Err(err) => log::error!("{}", err),
        //                 }
        //             }
        //             Err(err) => log::error!("{:?}", err),
        //         };

        //         match result_inside.map(|raw| {
        //             Measurement::new("inside", "", raw).to_http_req(CONFIG.server.endpoint)
        //         }) {
        //             Ok(value) => {
        //                 match wifi.http_post(CONFIG.server.host, CONFIG.server.port, &value) {
        //                     Ok(_) => {}
        //                     Err(err) => log::error!("{}", err),
        //                 }
        //             }
        //             Err(err) => log::error!("{:?}", err),
        //         };
        //     }
        //     Err(e) => log::error!("{:?}", e),
        // };

        drop(led.set_low());

        delay_ms!(2000u16);
    }
}
// use arduino_nano33iot::pac::interrupt;
// #[interrupt]
// fn TC5() {
//     // hal::pac::NVIC::
//     // hal::pac::NVIC::unpend(arduino_nano33iot::pac::interrupt::TC5);
//     unsafe {
//         hal::pac::TC5::ptr()
//             .as_ref()
//             .unwrap()
//             .count16()
//             .intflag
//             .modify(|_, w| w.ovf().set_bit());
//     }
//     log::info!("rtc int");
// }

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
