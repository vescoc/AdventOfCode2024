#![no_std]
#![no_main]

use defmt_rtt as _;

use embedded_io::{ErrorType, Read, Write};

use rp2040_hal as hal;

use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const XTAL_FREQ_HZ: u32 = 12_000_000;

struct Now(hal::timer::Timer);

impl embedded_aoc::Timer<u64, 1, 1_000_000> for Now {
    fn now(&self) -> hal::timer::Instant {
        self.0.get_counter()
    }
}

struct SerialWrapper<'a, B: UsbBus> {
    usb_device: UsbDevice<'a, B>,
    serial_port: SerialPort<'a, B>,
}

impl<'a, B: UsbBus> ErrorType for SerialWrapper<'a, B> {
    type Error = <SerialPort<'a, B> as ErrorType>::Error;
}

impl<B: UsbBus> Write for SerialWrapper<'_, B> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        Ok(self.serial_port.write(buf)?)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(self.serial_port.flush()?)
    }
}

impl<B: UsbBus> Read for SerialWrapper<'_, B> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        while !self.usb_device.poll(&mut [&mut self.serial_port]) {}

        Ok(self.serial_port.read(buf)?)
    }
}

#[panic_handler]
fn core_panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);

    loop {
        cortex_m::asm::wfe();
    }
}

#[hal::entry]
fn main() -> ! {
    extern "C" {
        static _stack_end: u32;
        static _stack_start: u32;
    }
    
    let mut pac = hal::pac::Peripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    defmt::info!("RP-PICO USB AoC 2024");

    let stack_low = &raw const _stack_end as u32;
    let stack_high = &raw const _stack_start as u32;

    defmt::info!("stack: [{} - {}]: {} bytes", stack_low, stack_high, stack_high - stack_low);
    
    let timer = Now(hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks));

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let serial_port = SerialPort::new(&usb_bus);

    let usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::default()
            .manufacturer("Vescoc Company")
            .product("Serial port")
            .serial_number("TEST")])
        .unwrap()
        .device_class(usbd_serial::USB_CLASS_CDC)
        .build();

    let mut serial = SerialWrapper {
        usb_device,
        serial_port,
    };

    embedded_aoc::run(&mut serial, &timer);
}

#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [hal::binary_info::EntryAddr; 5] = [
    hal::binary_info::rp_program_name!(c"AoC 2024 USB"),
    hal::binary_info::rp_cargo_version!(),
    hal::binary_info::rp_program_description!(c"AoC 2024 USB"),
    hal::binary_info::rp_program_url!(c"private"),
    hal::binary_info::rp_program_build_attribute!(),
];
