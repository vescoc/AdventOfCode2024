#![no_std]
#![no_main]

mod serial_logger;

use esp_hal::{clock, gpio, time, uart};

#[panic_handler]
fn core_panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("PANIC: {}", info);

    loop {
        core::hint::spin_loop();
    }
}

type Instant = fugit::Instant<u64, 1, 1_000_000>;

struct Now(time::Instant);

impl embedded_aoc::Timer<u64, 1, 1_000_000> for Now {
    fn now(&self) -> Instant {
        Instant::from_ticks(self.0.duration_since_epoch().as_micros())
    }
}

unsafe extern "C" {
    static _stack_end: u32;
    static _stack_start: u32;
}

#[esp_hal::main]
fn main() -> ! {
    let peripherals =
        esp_hal::init(esp_hal::Config::default().with_cpu_clock(clock::CpuClock::max()));

    let logger = uart::Uart::new(peripherals.UART0, uart::Config::default())
        .unwrap()
        .with_rx(peripherals.GPIO39)
        .with_tx(peripherals.GPIO40);
    let led_error = gpio::Output::new(
        peripherals.GPIO15,
        gpio::Level::Low,
        gpio::OutputConfig::default(),
    );

    serial_logger::SerialLogger::init(logger, led_error).ok();

    log::info!("ESP32S2 UART Aoc 2024");

    let stack_low = &raw const _stack_end as u32;
    let stack_high = &raw const _stack_start as u32;

    log::info!(
        "stack: [{stack_low:0x} - {stack_high:0x}]: 0x{0:0x} [{0}] bytes",
        stack_high - stack_low
    );

    let uart = esp_hal::uart::Uart::new(peripherals.UART1, esp_hal::uart::Config::default())
        .unwrap()
        .with_rx(peripherals.GPIO18)
        .with_tx(peripherals.GPIO17);

    let (rx, tx) = uart.split();

    let timer = Now(esp_hal::time::Instant::now());

    embedded_aoc::run((rx, tx), &timer, embedded_aoc::DummyHandler::default());
}
