use {
    arduino_nano33iot::{
        self as hal,
        clock::GenericClockController,
        gpio::{Floating, Input, Pa24, Pa25},
        pac::interrupt,
        UsbBus,
    },
    cortex_m::peripheral::NVIC,
    usb_device::{bus::UsbBusAllocator, prelude::*},
    usbd_serial::{SerialPort, USB_CLASS_CDC},
};

mod logger;

pub(super) static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
pub(super) static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
pub(super) static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

pub fn init(
    usb: hal::pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut hal::pac::PM,
    dm: Pa24<Input<Floating>>,
    dp: Pa25<Input<Floating>>,
    nvic: &mut hal::pac::NVIC,
) {
    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(hal::usb_allocator(usb, clocks, pm, dm, dp));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_SERIAL = Some(SerialPort::new(&bus_allocator));
        USB_BUS = Some(
            // Arduino VID, made up PID
            UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x2341, 0x3333))
                .manufacturer("olback")
                .product("Nano 33 IOT Serial Port")
                .serial_number("TEST")
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }

    unsafe {
        nvic.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }
}

pub fn init_logger() {
    drop(logger::UsbSerialLogger::init());
}

#[interrupt]
fn USB() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_SERIAL.as_mut().map(|serial| {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 16];
                let _ = serial.read(&mut buf);
            });
        });
    }
}
