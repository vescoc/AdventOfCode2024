#![feature(impl_trait_in_assoc_type)]
#![allow(clippy::used_underscore_binding)]
#![no_std]
#![no_main]

mod serial_logger;

use embassy_nrf::{
    bind_interrupts, buffered_uarte,
    gpio::{Level, Output, OutputDrive},
    pac, peripherals, uarte,
};

type Instant = fugit::Instant<u64, 1, 1_000_000>;
type Duration = fugit::Duration<u64, 1, 1_000_000>;

bind_interrupts!( struct Irqs {
    UARTE0 => buffered_uarte::InterruptHandler<peripherals::UARTE0>;
    UARTE1 => uarte::InterruptHandler<peripherals::UARTE1>;
});

unsafe extern "C" {
    static _stack_end: u32;
    static _stack_start: u32;
}

#[allow(clippy::struct_field_names)]
struct SimpleHandler<'d> {
    led_run: Output<'d>,
    led_invalid: Output<'d>,
    led_unsupported: Output<'d>,
}

impl embedded_aoc::Handler<u64, 1, 1_000_000> for SimpleHandler<'_> {
    fn started(&mut self, _: embedded_aoc::Day, _: Instant) {
        self.led_run.set_low();
        self.led_invalid.set_high();
        self.led_unsupported.set_high();
    }

    fn ended(&mut self, _: embedded_aoc::Day, _: Duration, _: &str, _: &str) {
        self.led_run.set_high();
        self.led_invalid.set_high();
        self.led_unsupported.set_high();
    }

    fn unsupported_day(&mut self) {
        self.led_run.set_high();
        self.led_invalid.set_high();
        self.led_unsupported.set_low();
    }

    fn invalid_input(&mut self) {
        self.led_run.set_high();
        self.led_invalid.set_low();
        self.led_unsupported.set_high();
    }
}

struct Now;

impl embedded_aoc::Timer<u64, 1, 1_000_000> for Now {
    fn now(&self) -> Instant {
        Instant::from_ticks(embassy_time::Instant::now().as_micros())
    }
}

#[panic_handler]
fn core_panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("PANIC: {}", info);

    loop {
        cortex_m::asm::wfe();
    }
}

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let stack_low = &raw const _stack_end as u32;
    let stack_high = &raw const _stack_start as u32;

    let peripherals = embassy_nrf::init(embassy_nrf::config::Config::default());

    pac::CLOCK.tasks_hfclkstart().write_value(1);
    while pac::CLOCK.events_hfclkstarted().read() != 1 {}

    let led_run = Output::new(peripherals.P0_16, Level::High, OutputDrive::Standard);
    let led_invalid = Output::new(peripherals.P0_24, Level::High, OutputDrive::Standard);
    let led_unsupported = Output::new(peripherals.P0_06, Level::High, OutputDrive::Standard);

    let handler = SimpleHandler {
        led_run,
        led_invalid,
        led_unsupported,
    };

    let led_error = Output::new(peripherals.P0_13, Level::Low, OutputDrive::Standard);

    let timer = Now;

    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD115200;

    static TX_BUFFER: static_cell::StaticCell<[u8; 256]> = static_cell::StaticCell::new();
    static RX_BUFFER: static_cell::StaticCell<[u8; 256]> = static_cell::StaticCell::new();

    let serial = buffered_uarte::BufferedUarte::new(
        peripherals.UARTE0,
        peripherals.TIMER0,
        peripherals.PPI_CH0,
        peripherals.PPI_CH1,
        peripherals.PPI_GROUP0,
        Irqs,
        peripherals.P1_10,
        peripherals.P1_03,
        config.clone(),
        RX_BUFFER.init_with(|| [0; 256]),
        TX_BUFFER.init_with(|| [0; 256]),
    );

    let logger = uarte::Uarte::new(
        peripherals.UARTE1,
        Irqs,
        peripherals.P1_11,
        peripherals.P1_12,
        config,
    );

    serial_logger::SerialLogger::init(logger, led_error).ok();

    log::info!("ARDUINO-NANO33BLESENSE EMBASSY UART AoC 2024");
    log::info!(
        "stack: [0x{stack_low:0x} - 0x{stack_high:0x}]: {0} [0x{0:0x}] bytes",
        stack_high - stack_low
    );

    let (rx, tx) = serial.split();

    embedded_aoc::run((rx, tx), &timer, handler).await;
}
