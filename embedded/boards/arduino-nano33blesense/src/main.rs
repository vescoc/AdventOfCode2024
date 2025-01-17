#![no_std]
#![no_main]

use core::cell::RefCell;

mod serial_logger;
mod serial_splitter;
mod timestamp;

use nrf52840_hal as hal;

use hal::{clocks, gpio, timer, uarte, Uarte};

use embedded_hal::digital::OutputPin;

type Instant = fugit::Instant<u64, 1, 1_000_000>;
type Duration = fugit::Duration<u64, 1, 1_000_000>;

unsafe extern "C" {
    static _stack_end: u32;
    static _stack_start: u32;
}

#[allow(clippy::struct_field_names)]
struct SimpleHandler<P1, P2, P3>
where
    P1: OutputPin,
    P2: OutputPin,
    P3: OutputPin,
{
    led_run: P1,
    led_invalid: P2,
    led_unsupported: P3,
}

impl<P1, P2, P3> embedded_aoc::Handler<u64, 1, 1_000_000> for SimpleHandler<P1, P2, P3>
where
    P1: OutputPin,
    P2: OutputPin,
    P3: OutputPin,
{
    fn started(&mut self, _: embedded_aoc::Day, _: Instant) {
        self.led_run.set_low().ok();
        self.led_invalid.set_high().ok();
        self.led_unsupported.set_high().ok();
    }

    fn ended(&mut self, _: embedded_aoc::Day, _: Duration, _: &str, _: &str) {
        self.led_run.set_high().ok();
        self.led_invalid.set_high().ok();
        self.led_unsupported.set_high().ok();
    }

    fn unsupported_day(&mut self) {
        self.led_run.set_high().ok();
        self.led_invalid.set_high().ok();
        self.led_unsupported.set_low().ok();
    }

    fn invalid_input(&mut self) {
        self.led_run.set_high().ok();
        self.led_invalid.set_low().ok();
        self.led_unsupported.set_high().ok();
    }
}

struct Now(timestamp::Timestamp);

impl embedded_aoc::Timer<u64, 1, 1_000_000> for Now {
    fn now(&self) -> Instant {
        self.0.now()
    }
}

#[panic_handler]
fn core_panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("PANIC: {}", info);

    loop {
        cortex_m::asm::wfe();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let stack_low = &raw const _stack_end as u32;
    let stack_high = &raw const _stack_start as u32;

    let dp = hal::pac::Peripherals::take().unwrap();
    let _clocks = clocks::Clocks::new(dp.CLOCK).enable_ext_hfosc();

    let port0 = gpio::p0::Parts::new(dp.P0);
    let led_run = port0.p0_16.into_push_pull_output(gpio::Level::High);
    let led_invalid = port0.p0_24.into_push_pull_output(gpio::Level::High);
    let led_unsupported = port0.p0_06.into_push_pull_output(gpio::Level::High);

    let led_error = port0.p0_13.into_push_pull_output(gpio::Level::Low);

    let handler = SimpleHandler {
        led_run,
        led_invalid,
        led_unsupported,
    };

    let timer = Now(timestamp::Timestamp::new(timer::Timer::periodic(dp.TIMER2)));

    let port1 = gpio::p1::Parts::new(dp.P1);
    let serial = RefCell::new(Uarte::new(
        dp.UARTE0,
        uarte::Pins {
            rxd: port1.p1_10.into_floating_input().degrade(),
            txd: port1
                .p1_03
                .into_push_pull_output(gpio::Level::Low)
                .degrade(),
            cts: None,
            rts: None,
        },
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200,
    ));

    let logger = Uarte::new(
        dp.UARTE1,
        uarte::Pins {
            rxd: port1.p1_11.into_floating_input().degrade(),
            txd: port1
                .p1_12
                .into_push_pull_output(gpio::Level::Low)
                .degrade(),
            cts: None,
            rts: None,
        },
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200,
    );

    serial_logger::SerialLogger::init(logger, led_error.degrade()).ok();

    log::info!("ARDUINO-NANO33BLESENSE UART AoC 2024");
    log::info!(
        "stack: [0x{stack_low:0x} - 0x{stack_high:0x}]: {0} [0x{0:0x}] bytes",
        stack_high - stack_low
    );

    let (rx, tx) = serial_splitter::split(&serial, timer::Timer::new(dp.TIMER0));

    embedded_aoc::run((rx, tx), &timer, handler);
}
