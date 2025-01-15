#![no_std]
#![no_main]

mod timestamp;
mod usb;

use defmt_rtt as _;

use embedded_time::rate::*;

use stm32f3xx_hal as hal;

use hal::gpio::{self, Output, Pin, PushPull};
use hal::pac;
use hal::prelude::*;
use hal::timer;

use usb_device::prelude::*;

type Instant = fugit::Instant<u64, 1, 1_000_000>;
type Duration = fugit::Duration<u64, 1, 1_000_000>;

#[panic_handler]
fn core_panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);

    loop {
        cortex_m::asm::bkpt();
    }
}

struct Now(timestamp::Timestamp<pac::TIM2>);

impl embedded_aoc::Timer<u64, 1, 1_000_000> for Now {
    fn now(&self) -> Instant {
        self.0.now()
    }
}

#[allow(clippy::struct_field_names)]
struct SimpleHandler<GPIO1, N1, GPIO2, N2, GPIO3, N3> {
    led_run: Pin<GPIO1, N1, Output<PushPull>>,
    led_invalid: Pin<GPIO2, N2, Output<PushPull>>,
    led_unsupported: Pin<GPIO3, N3, Output<PushPull>>,
}

impl<GPIO1, N1, GPIO2, N2, GPIO3, N3> embedded_aoc::Handler<u64, 1, 1_000_000>
    for SimpleHandler<GPIO1, N1, GPIO2, N2, GPIO3, N3>
where
    GPIO1: gpio::marker::Gpio,
    N1: gpio::marker::Index,
    GPIO2: gpio::marker::Gpio,
    N2: gpio::marker::Index,
    GPIO3: gpio::marker::Gpio,
    N3: gpio::marker::Index,
{
    fn started(&mut self, _: embedded_aoc::Day, _: Instant) {
        self.led_run.set_high().ok();
        self.led_invalid.set_low().ok();
        self.led_unsupported.set_low().ok();
    }

    fn ended(&mut self, _: embedded_aoc::Day, _: Duration, _: &str, _: &str) {
        self.led_run.set_low().ok();
        self.led_invalid.set_low().ok();
        self.led_unsupported.set_low().ok();
    }

    fn unsupported_day(&mut self) {
        self.led_run.set_low().ok();
        self.led_invalid.set_low().ok();
        self.led_unsupported.set_high().ok();
    }

    fn invalid_input(&mut self) {
        self.led_run.set_low().ok();
        self.led_invalid.set_high().ok();
        self.led_unsupported.set_low().ok();
    }
}

extern "C" {
    static _stack_end: u32;
    static _stack_start: u32;
}

#[cortex_m_rt::entry]
#[allow(clippy::similar_names)]
fn main() -> ! {
    defmt::info!("STM32 STM32F3DISCOVERY USB AoC 2024");

    let stack_low = &raw const _stack_end as u32;
    let stack_high = &raw const _stack_start as u32;

    defmt::info!(
        "stack: [{} - {}]: {} bytes",
        stack_low,
        stack_high,
        stack_high - stack_low
    );

    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz()) // 48
        .pclk1(24.MHz())
        .pclk2(24.MHz())
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid());

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let usb_dm = gpioa
        .pa11
        .into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let usb_dp = gpioa
        .pa12
        .into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

    let usb = usb::Peripheral {
        usb: dp.USB,
        pin_dm: usb_dm,
        pin_dp: usb_dp,
    };

    let usb_bus = usb::UsbBus::new(usb);

    let serial_port = usbd_serial::SerialPort::new(&usb_bus);

    let usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[usb_device::device::StringDescriptors::default()
            .manufacturer("Vescoc Company")
            .product("Serial port")
            .serial_number("TEST")])
        .unwrap()
        .device_class(usbd_serial::USB_CLASS_CDC)
        .build();

    let serial = serial_port_splitter::Splitter::new(usb_device, serial_port);

    let (rx, tx) = serial.split();

    let timer = Now(timestamp::Timestamp::new(timer::Timer::new(
        dp.TIM2,
        clocks,
        &mut rcc.apb1,
    )));

    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let mut led_run = gpioe
        .pe15
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_invalid = gpioe
        .pe14
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_unsupported = gpioe
        .pe13
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    led_run.set_low().ok();
    led_invalid.set_low().ok();
    led_unsupported.set_low().ok();

    let handler = SimpleHandler {
        led_run,
        led_invalid,
        led_unsupported,
    };

    embedded_aoc::run((rx, tx), &timer, handler);
}
