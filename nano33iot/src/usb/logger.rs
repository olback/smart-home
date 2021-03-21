use {
    super::{USB_BUS, USB_SERIAL},
    cortex_m,
    log::{LevelFilter, Metadata, Record, SetLoggerError},
};

static LOGGER: UsbSerialLogger = UsbSerialLogger;

pub(super) struct UsbSerialLogger;

impl UsbSerialLogger {
    pub(super) fn init() -> Result<(), SetLoggerError> {
        unsafe { log::set_logger_racy(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace)) }
    }
}

impl log::Log for UsbSerialLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::STATIC_MAX_LEVEL
    }

    fn log(&self, record: &Record) {
        cortex_m::interrupt::free(|_| unsafe {
            USB_BUS.as_mut().map(|_| {
                USB_SERIAL.as_mut().map(|serial| {
                    let _ = serial.write(
                        alloc::format!(
                            "[{}] {}#{}: {}\n",
                            record.level(),
                            record.file().unwrap(),
                            record.line().unwrap(),
                            record.args()
                        )
                        .as_bytes(),
                    );
                })
            })
        });
    }

    fn flush(&self) {}
}
