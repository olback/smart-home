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

mod usb;
mod wifi;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

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
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut led = pins.led_sck.into_open_drain_output(&mut pins.port);
    let mut sensor = pins.a7.into_readable_open_drain_output(&mut pins.port);

    usb::init(
        peripherals.USB,
        &mut clocks,
        &mut peripherals.PM,
        pins.usb_dm,
        pins.usb_dp,
        &mut core.NVIC,
    );
    usb::init_logger();

    loop {
        led.toggle();
        delay.delay_ms(2000u16);
        let result = dht22::Reading::read(&mut delay, &mut sensor);
        log::info!("{:#?}", result);
    }
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
