use core::cell::RefCell;
use core::fmt::Write;

use embedded_hal::digital::OutputPin;

use crate::hal;

use hal::{gpio, Uarte};

#[allow(clippy::upper_case_acronyms)]
type UARTE = hal::pac::UARTE1;

static LOGGER: SerialLogger = SerialLogger::new();

struct SerialLoggerInner {
    serial: Uarte<UARTE>,
    led_error: gpio::Pin<gpio::Output<gpio::PushPull>>,
}

pub struct SerialLogger(critical_section::Mutex<RefCell<Option<SerialLoggerInner>>>);

impl SerialLogger {
    const fn new() -> Self {
        Self(critical_section::Mutex::new(RefCell::new(None)))
    }

    pub fn init(
        serial: Uarte<UARTE>,
        led_error: gpio::Pin<gpio::Output<gpio::PushPull>>,
    ) -> Result<(), log::SetLoggerError> {
        critical_section::with(|cs| {
            LOGGER
                .0
                .replace(cs, Some(SerialLoggerInner { serial, led_error }))
        });

        log::set_max_level(log::LevelFilter::Trace);
        log::set_logger(&LOGGER)
    }
}

impl log::Log for SerialLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            critical_section::with(|cs| {
                if let Some(SerialLoggerInner { serial, led_error }) =
                    LOGGER.0.borrow_ref_mut(cs).as_mut()
                {
                    let level = record.level();
                    if level == log::Level::Error {
                        led_error.set_high().ok();
                    }

                    writeln!(serial, "{}: {}", level, record.args()).ok();
                }
            });
        }
    }

    fn flush(&self) {}
}
