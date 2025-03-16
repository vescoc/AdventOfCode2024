#![no_std]
#![no_main]

mod timestamp;

use defmt_rtt as _;

use nrf52840_hal as hal;

use hal::{clocks, gpio, timer, usbd};

use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

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
    defmt::error!("PANIC: {}", info);

    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("NRF52840-DK USB AoC 2024");

    let stack_low = &raw const _stack_end as u32;
    let stack_high = &raw const _stack_start as u32;

    defmt::info!(
        "stack: [{} - {}]: {} bytes",
        stack_low,
        stack_high,
        stack_high - stack_low
    );

    let pac = hal::pac::Peripherals::take().unwrap();
    let clocks = clocks::Clocks::new(pac.CLOCK);
    let clocks = clocks.enable_ext_hfosc();

    let port0 = gpio::p0::Parts::new(pac.P0);
    let led_run = port0.p0_13.into_push_pull_output(gpio::Level::High);
    let led_invalid = port0.p0_14.into_push_pull_output(gpio::Level::High);
    let led_unsupported = port0.p0_15.into_push_pull_output(gpio::Level::High);

    let handler = SimpleHandler {
        led_run,
        led_invalid,
        led_unsupported,
    };

    let usb_bus =
        UsbBusAllocator::new(usbd::Usbd::new(usbd::UsbPeripheral::new(pac.USBD, &clocks)));
    let serial_port = SerialPort::new(&usb_bus);
    let usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::default()
            .manufacturer("Vescoc Company")
            .product("Serial port")
            .serial_number("TEST")])
        .unwrap()
        .device_class(usbd_serial::USB_CLASS_CDC)
        .max_packet_size_0(64)
        .unwrap()
        .build();

    let serial = serial_port_splitter::Splitter::new(usb_device, serial_port);

    let (rx, tx) = serial.split();

    let timer = Now(timestamp::Timestamp::new(timer::Timer::periodic(
        pac.TIMER2,
    )));

    embedded_aoc::run((rx, tx), &timer, handler);
}
