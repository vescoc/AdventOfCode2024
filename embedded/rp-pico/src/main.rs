#![no_std]
#![no_main]

use defmt_rtt as _;

use rp2040_hal as hal;

use usb_device::class_prelude::*;

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

#[panic_handler]
fn core_panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);

    loop {
        cortex_m::asm::wfe();
    }
}

#[hal::entry]
fn main() -> ! {
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
    
    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    defmt::info!("AoC 2024 USB");

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let now = Now(timer);

    embedded_aoc::usb(&now, &usb_bus);
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
