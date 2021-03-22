#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]

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
};

mod config;
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

    usb::init(
        peripherals.USB,
        &mut clocks,
        &mut peripherals.PM,
        pins.usb_dm,
        pins.usb_dp,
        &mut core.NVIC,
    );
    usb::init_logger();

    delay_ms!(5000u16);

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

    let mut client = loop {
        match nina_wifi.new_client() {
            Ok(c) => {
                log::info!("[WiFi NINA] Client OK");
                break c;
            }
            Err(ref e) => log::error!("[WiFi NINA] Failed to get new client {:?}", e),
        }
    };

    loop {
        match client.connect_ipv4(
            &mut nina_wifi,
            no_std_net::Ipv4Addr::new(192, 168, 43, 178),
            config::CONFIG.server.port,
            wifi_nina::types::ProtocolMode::Tcp,
        ) {
            Ok(_) => {
                log::info!("[WiFi NINA] Connected to server");
                break;
            }
            Err(ref e) => log::error!("Error connecting to server {:?}", e),
        }
    }

    loop {
        delay_ms!(2000u16);
        let _ = led.set_high();
        let result_outside = dht22::Reading::read(delay!(), &mut sensor_outside);
        let result_inside = dht22::Reading::read(delay!(), &mut sensor_inside);
        log::info!("Outside: {:#?}", result_outside);
        delay_ms!(10u8);
        log::info!("Inside: {:#?}", result_inside);
        let _ = led.set_low();
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    log::error!("{}", info);
    loop {}
}
