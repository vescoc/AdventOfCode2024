use core::cell::RefCell;
use core::fmt::{self, Write as _};

use embassy_nrf::{gpio, uarte};

#[allow(clippy::upper_case_acronyms)]
type UARTE = embassy_nrf::peripherals::UARTE1;

static LOGGER: SerialLogger = SerialLogger::new();

struct SerialLoggerInner {
    serial: uarte::Uarte<'static, UARTE>,
    led_error: gpio::Output<'static>,
}

pub struct SerialLogger(critical_section::Mutex<RefCell<Option<SerialLoggerInner>>>);

impl SerialLogger {
    const fn new() -> Self {
        Self(critical_section::Mutex::new(RefCell::new(None)))
    }

    pub fn init(
        serial: uarte::Uarte<'static, UARTE>,
        led_error: gpio::Output<'static>,
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
                        led_error.set_high();
                    }

                    writeln!(Wrapper(serial), "{}: {}", level, record.args()).ok();
                }
            });
        }
    }

    fn flush(&self) {}
}

struct Wrapper<W>(W);

impl<W: embedded_io_async::Write> fmt::Write for Wrapper<W> {
    fn write_str(&mut self, value: &str) -> Result<(), fmt::Error> {
        let mut buf = value.as_bytes();
        while !buf.is_empty() {
            match embassy_futures::block_on(self.0.write(value.as_bytes())) {
                Ok(count) => {
                    buf = &buf[count..];
                }
                _ => {
                    return Err(fmt::Error);
                }
            }
        }
        Ok(())
    }
}
