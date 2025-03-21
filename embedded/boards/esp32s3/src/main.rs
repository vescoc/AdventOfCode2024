#![no_std]
#![no_main]

use esp_backtrace as _;

use esp_hal::clock::CpuClock;

use log::info;

struct Now;

impl embedded_aoc::Timer<u64, 1, 1_000_000> for Now {
    fn now(&self) -> esp_hal::time::Instant {
        esp_hal::time::now()
    }
}

unsafe extern "C" {
    static _stack_end: u32;
    static _stack_start: u32;
}

#[esp_hal::main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    info!("ESP32S3 UART Aoc 2024");

    let stack_low = &raw const _stack_end as u32;
    let stack_high = &raw const _stack_start as u32;

    info!(
        "stack: [{stack_low:0x} - {stack_high:0x}]: 0x{0:0x} [{0}] bytes",
        stack_high - stack_low
    );

    let uart = esp_hal::uart::Uart::new(peripherals.UART1, esp_hal::uart::Config::default())
        .unwrap()
        .with_rx(peripherals.GPIO18)
        .with_tx(peripherals.GPIO17);

    let (rx, tx) = uart.split();

    let timer = Now;

    embedded_aoc::run((rx, tx), &timer, embedded_aoc::DummyHandler::default());
}
