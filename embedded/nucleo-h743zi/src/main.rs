#![no_std]
#![no_main]

use core::cell::RefCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU32, Ordering};

use defmt_rtt as _;

use cortex_m::interrupt::Mutex;

use stm32h7xx_hal as hal;

use hal::gpio::{Output, Pin};
use hal::pac;
use hal::prelude::*;
use hal::rcc::rec::UsbClkSel;
use hal::timer;
use hal::usb_hs::{UsbBus, USB2};

use pac::interrupt;

use fugit::{Duration, Instant};

use usb_device::prelude::*;

#[panic_handler]
fn core_panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);

    loop {
        cortex_m::asm::wfe();
    }
}

struct Now;

impl embedded_aoc::Timer<u64, 1, 1_000_000> for Now {
    fn now(&self) -> fugit::Instant<u64, 1, 1_000_000> {
        Instant::<u64, 1, 1_000_000>::from_ticks(timestamp())
    }
}

extern "C" {
    static _stack_end: u32;
    static _stack_start: u32;
}

static mut EP_MEMORY: MaybeUninit<[u32; 1024]> = MaybeUninit::uninit();

static OVERFLOWS: AtomicU32 = AtomicU32::new(0);
static TIMER: Mutex<RefCell<Option<timer::Timer<pac::TIM2>>>> = Mutex::new(RefCell::new(None));

struct SimpleHandler<
    const P1: char,
    const N1: u8,
    const P2: char,
    const N2: u8,
    const P3: char,
    const N3: u8,
> {
    led1: Pin<P1, N1, Output>,
    led2: Pin<P2, N2, Output>,
    led3: Pin<P3, N3, Output>,
}

impl<const P1: char, const N1: u8, const P2: char, const N2: u8, const P3: char, const N3: u8>
    embedded_aoc::Handler<u64, 1, 1_000_000> for SimpleHandler<P1, N1, P2, N2, P3, N3>
{
    fn started(&mut self, _: embedded_aoc::Day, _: Instant<u64, 1, 1_000_000>) {
        self.led1.set_high();
        self.led2.set_low();
        self.led3.set_low();
    }

    fn ended(&mut self, _: embedded_aoc::Day, _: Duration<u64, 1, 1_000_000>, _: &str, _: &str) {
        self.led1.set_low();
        self.led2.set_low();
        self.led3.set_low();
    }

    fn unsupported_day(&mut self) {
        self.led1.set_low();
        self.led2.set_low();
        self.led3.set_high();
    }

    fn invalid_input(&mut self) {
        self.led1.set_low();
        self.led2.set_high();
        self.led3.set_low();
    }
}

#[cortex_m_rt::entry]
#[allow(clippy::similar_names)]
fn main() -> ! {
    defmt::info!("STM32 NUCLEO-H743ZI");

    let stack_low = &raw const _stack_end as u32;
    let stack_high = &raw const _stack_start as u32;

    defmt::info!(
        "stack: [{} - {}]: {} bytes",
        stack_low,
        stack_high,
        stack_high - stack_low
    );

    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    let mut ccdr = rcc.sys_ck(400.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    let _ = ccdr.clocks.hsi48_ck().expect("HSI48 must run");
    ccdr.peripheral.kernel_usb_clk_mux(UsbClkSel::Hsi48);

    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    let mut led1 = gpiob.pb0.into_push_pull_output();
    let mut led2 = gpiob.pb7.into_push_pull_output();
    let mut led3 = gpiob.pb14.into_push_pull_output();

    led1.set_low();
    led2.set_low();
    led3.set_low();

    let mut timer = dp
        .TIM2
        .tick_timer(1.MHz(), ccdr.peripheral.TIM2, &ccdr.clocks);
    timer.listen(timer::Event::TimeOut);

    cortex_m::interrupt::free(|cs| {
        TIMER.borrow(cs).replace(Some(timer));
    });

    unsafe {
        cp.NVIC.set_priority(interrupt::TIM2, 1);
        pac::NVIC::unmask(interrupt::TIM2);
    }

    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);

    let pin_dm = gpioa.pa11.into_alternate();
    let pin_dp = gpioa.pa12.into_alternate();

    let usb = USB2::new(
        dp.OTG2_HS_GLOBAL,
        dp.OTG2_HS_DEVICE,
        dp.OTG2_HS_PWRCLK,
        pin_dm,
        pin_dp,
        ccdr.peripheral.USB2OTG,
        &ccdr.clocks,
    );

    let usb_bus = UsbBus::new(usb, unsafe {
        let buf: &mut [MaybeUninit<u32>; 1024] = &mut *(&raw mut EP_MEMORY).cast();
        for value in buf.iter_mut() {
            value.as_mut_ptr().write(0);
        }

        let buf: &mut MaybeUninit<[u32; 1024]> = &mut *(&raw mut EP_MEMORY).cast();
        buf.assume_init_mut()
    });

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

    let timer = Now;

    let handler = SimpleHandler { led1, led2, led3 };

    embedded_aoc::run((rx, tx), &timer, handler);
}

fn timestamp() -> u64 {
    let overflows = u64::from(OVERFLOWS.load(Ordering::SeqCst));
    let counter = cortex_m::interrupt::free(|cs| {
        u64::from(TIMER.borrow(cs).borrow().as_ref().unwrap().counter())
    });
    (overflows << 32) + counter
}

#[interrupt]
fn TIM2() {
    OVERFLOWS.fetch_add(1, Ordering::SeqCst);
    cortex_m::interrupt::free(|cs| {
        TIMER.borrow(cs).borrow_mut().as_mut().unwrap().clear_irq();
    });
}
