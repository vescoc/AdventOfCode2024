#![feature(impl_trait_in_assoc_type)]

#![allow(clippy::used_underscore_binding)]

#![no_std]
#![no_main]

use esp_backtrace as _;

use esp_hal::clock::CpuClock;
use esp_hal::uart::Uart;
use esp_hal::timer::timg::TimerGroup;

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

#[esp_hal_embassy::main]
async fn main(_spawner: embassy_executor::Spawner) -> ! {
    esp_println::logger::init_logger_from_env();

    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    info!("ESP32C3 EMBASSY UART AoC 2024");

    let stack_low = &raw const _stack_end as u32;
    let stack_high = &raw const _stack_start as u32;

    info!("stack: [{stack_low:0x} - {stack_high:0x}]: 0x{0:0x} [{0}] bytes", stack_high - stack_low);
    
    let uart = Uart::new(peripherals.UART1,
                         peripherals.GPIO18, // rx (tx on USB - serial adapter)
                         peripherals.GPIO19) // tx (rx on USB - serial adapter)
        .unwrap()
        .into_async();
    
    let (rx, tx) = uart.split();

    let timer = Now;

    embedded_aoc::run((rx, tx), &timer, embedded_aoc::DummyHandler::default()).await
}
