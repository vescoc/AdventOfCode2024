#![no_std]
#![no_main]

use core::fmt::{self, Write};

use heapless::{String as HLString, Vec as HLVec};

use defmt_rtt as _;

use rp235x_hal as hal;

use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

type Vec<T> = HLVec<T, { 1024 * 32 }>;
type String = HLString<1024>;
type PartResult = HLString<64>;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

#[panic_handler]
fn core_panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);

    loop {
        cortex_m::asm::wfe();
    }
}

const XTAL_FREQ_HZ: u32 = 12_000_000;

const START_INPUT_TAG: &str = "START INPUT DAY: ";
const END_INPUT_TAG: &str = "END INPUT";

#[derive(defmt::Format)]
enum Day {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl Day {
    fn to_string(result: &mut PartResult, value: impl fmt::Display) -> Result<(), fmt::Error> {
        write!(result, "{value}")
    }

    fn solve_1(&self, result: &mut PartResult, input: &str) -> Result<(), fmt::Error> {
        match self {
            Day::Day01 => Self::to_string(result, day01::solve_1(input)),
            Day::Day02 => Self::to_string(result, day02::solve_1(input)),
            Day::Day03 => Self::to_string(result, day03::solve_1(input)),
            Day::Day04 => Self::to_string(result, day04::solve_1(input)),
            Day::Day05 => Self::to_string(result, day05::solve_1(input)),
            Day::Day06 => Self::to_string(result, day06::solve_1(input)),
            Day::Day07 => Self::to_string(result, day07::solve_1(input)),
            Day::Day08 => Self::to_string(result, day08::solve_1(input)),
            Day::Day09 => Self::to_string(result, day09::solve_1(input)),
            Day::Day10 => Self::to_string(result, day10::solve_1(input)),
            Day::Day11 => Self::to_string(result, day11::solve_1(input)),
            Day::Day12 => Self::to_string(result, day12::solve_1(input)),
            Day::Day13 => Self::to_string(result, day13::solve_1(input)),
            Day::Day14 => Self::to_string(result, day14::solve_1::<{ day14::WIDTH }, { day14::HEIGHT }>(input)),
            Day::Day15 => Self::to_string(result, day15::solve_1(input)),
            Day::Day16 => Self::to_string(result, day16::solve_1(input)),
            Day::Day17 => Self::to_string(result, day17::solve_1(input)),
            Day::Day18 => Self::to_string(result, day18::solve_1(input)),
            Day::Day19 => Self::to_string(result, day19::solve_1(input)),
            Day::Day20 => Self::to_string(result, day20::solve_1(input)),
            Day::Day21 => Self::to_string(result, day21::solve_1(input)),
            Day::Day22 => Self::to_string(result, day22::solve_1(input)),
            Day::Day23 => Self::to_string(result, day23::solve_1(input)),
            Day::Day24 => Self::to_string(result, day24::solve_1(input)),
            Day::Day25 => Self::to_string(result, day25::solve_1(input)),
        }
    }

    fn solve_2(&self, result: &mut PartResult, input: &str) -> Result<(), fmt::Error> {
        match self {
            Day::Day01 => Self::to_string(result, day01::solve_2(input)),
            Day::Day02 => Self::to_string(result, day02::solve_2(input)),
            Day::Day03 => Self::to_string(result, day03::solve_2(input)),
            Day::Day04 => Self::to_string(result, day04::solve_2(input)),
            Day::Day05 => Self::to_string(result, day05::solve_2(input)),
            Day::Day06 => Self::to_string(result, day06::solve_2(input)),
            Day::Day07 => Self::to_string(result, day07::solve_2(input)),
            Day::Day08 => Self::to_string(result, day08::solve_2(input)),
            Day::Day09 => Self::to_string(result, day09::solve_2(input)),
            Day::Day10 => Self::to_string(result, day10::solve_2(input)),
            Day::Day11 => Self::to_string(result, day11::solve_2(input)),
            Day::Day12 => Self::to_string(result, day12::solve_2(input)),
            Day::Day13 => Self::to_string(result, day13::solve_2(input)),
            Day::Day14 => Self::to_string(result, day14::solve_2(input)),
            Day::Day15 => Self::to_string(result, day15::solve_2(input)),
            Day::Day16 => Self::to_string(result, day16::solve_2(input)),
            Day::Day17 => Self::to_string(result, day17::solve_2(input)),
            Day::Day18 => Self::to_string(result, day18::solve_2(input)),
            Day::Day19 => Self::to_string(result, day19::solve_2(input)),
            Day::Day20 => Self::to_string(result, day20::solve_2(input)),
            Day::Day21 => Self::to_string(result, day21::solve_2(input)),
            Day::Day22 => Self::to_string(result, day22::solve_2(input)),
            Day::Day23 => Self::to_string(result, day23::solve_2(input)),
            Day::Day24 => Self::to_string(result, day24::solve_2(input)),
            Day::Day25 => Self::to_string(result, day25::solve_2(input)),
        }
    }
}

impl core::str::FromStr for Day {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.chars().take(2).try_fold(0, |acc, digit| {
            match digit {
                '0'..='9' => Some(acc * 10 + digit as u32 - '0' as u32),
                _ => None,                    
            }
        })
        {
            Some(1) => Ok(Day::Day01),
            Some(2) => Ok(Day::Day02),
            Some(3) => Ok(Day::Day03),
            Some(4) => Ok(Day::Day04),
            Some(5) => Ok(Day::Day05),
            Some(6) => Ok(Day::Day06),
            Some(7) => Ok(Day::Day07),
            Some(8) => Ok(Day::Day08),
            Some(9) => Ok(Day::Day09),
            Some(10) => Ok(Day::Day10),
            Some(11) => Ok(Day::Day11),
            Some(12) => Ok(Day::Day12),
            Some(13) => Ok(Day::Day13),
            Some(14) => Ok(Day::Day14),
            Some(15) => Ok(Day::Day15),
            Some(16) => Ok(Day::Day16),
            Some(17) => Ok(Day::Day17),
            Some(18) => Ok(Day::Day18),
            Some(19) => Ok(Day::Day19),
            Some(20) => Ok(Day::Day20),
            Some(21) => Ok(Day::Day21),
            Some(22) => Ok(Day::Day22),
            Some(23) => Ok(Day::Day23),
            Some(24) => Ok(Day::Day24),
            Some(25) => Ok(Day::Day25),
            Some(_) => Err("invalid day"),
            None => Err("invalid number"),
        }
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let day = match self {
            Day::Day01 => 1,
            Day::Day02 => 2,
            Day::Day03 => 3,
            Day::Day04 => 4,
            Day::Day05 => 5,
            Day::Day06 => 6,
            Day::Day07 => 7,
            Day::Day08 => 8,
            Day::Day09 => 9,
            Day::Day10 => 10,
            Day::Day11 => 11,
            Day::Day12 => 12,
            Day::Day13 => 13,
            Day::Day14 => 14,
            Day::Day15 => 15,
            Day::Day16 => 16,
            Day::Day17 => 17,
            Day::Day18 => 18,
            Day::Day19 => 19,
            Day::Day20 => 20,
            Day::Day21 => 21,
            Day::Day22 => 22,
            Day::Day23 => 23,
            Day::Day24 => 24,
            Day::Day25 => 25,
        };

        write!(f, "{day:02}")
    }
}

#[allow(clippy::too_many_lines)]
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

    let timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    defmt::info!("AoC 2024 day XX");

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USB,
        pac.USB_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::default()
            .manufacturer("Vescoc Company")
            .product("Serial port")
            .serial_number("TEST")])
        .unwrap()
        .device_class(usbd_serial::USB_CLASS_CDC)
        .build();

    let mut buffer = Vec::new();
    loop {
        buffer.clear();
        loop {
            if usb_device.poll(&mut [&mut serial]) {
                let mut buf = [0u8; 1024];
                match serial.read(&mut buf) {
                    Err(_) | Ok(0) => {}
                    Ok(count) => {
                        if buffer.extend_from_slice(&buf[..count]).is_err() {
                            defmt::warn!("buffer overflow");
                            break;
                        }

                        if let Ok(input) = core::str::from_utf8(&buffer) {
                            match (input.find(START_INPUT_TAG), input.find(END_INPUT_TAG)) {
                                (Some(start_position), Some(end_position)) => {
                                    let Ok(day) = input[start_position + START_INPUT_TAG.len()..]
                                        .parse::<Day>()
                                    else {
                                        defmt::warn!("must give a target day, resetting input");
                                        break;
                                    };

                                    let input = input
                                        [start_position + START_INPUT_TAG.len() + 2..end_position]
                                        .trim();

                                    defmt::info!("start working on {}", day);

                                    let mut part_1 = PartResult::new();
                                    let mut part_2 = PartResult::new();

                                    let start = timer.get_counter();

                                    if day.solve_1(&mut part_1, input).is_err() {
                                        defmt::warn!("part_1: buffer overflow");
                                        break;
                                    }

                                    if day.solve_2(&mut part_2, input).is_err() {
                                        defmt::warn!("part_2: buffer overflow");
                                        break;
                                    }

                                    let elapsed = timer.get_counter() - start;

                                    defmt::info!("end working on {}", day);

                                    let mut result = String::new();
                                    {
                                        result.clear();
                                        write!(&mut result, "[{day}] part 1: {part_1}").unwrap();
                                        defmt::info!("{}", result.as_str());
                                    }
                                    {
                                        result.clear();
                                        write!(&mut result, "[{day}] part 2: {part_2}").unwrap();
                                        defmt::info!("{}", result.as_str());
                                    }
                                    {
                                        result.clear();
                                        write!(
                                            &mut result,
                                            "[{day}] elapsed: {}ms ({}us)",
                                            elapsed.to_millis(),
                                            elapsed.to_micros()
                                        )
                                        .unwrap();
                                        defmt::info!("{}", result.as_str());
                                    }

                                    {
                                        result.clear();
                                        write!(&mut result, "\r\nday {day}\r\npart 1: {part_1}\r\npart 2: {part_2}\r\nelapsed: {}ms ({}us)\r\n", elapsed.to_millis(), elapsed.to_micros()).unwrap();

                                        let mut max_retry = 0;
                                        let mut buf = result.as_str().as_bytes();
                                        while !buf.is_empty() {
                                            match serial.write(buf) {
                                                Ok(len) => buf = &buf[len..],
                                                Err(UsbError::WouldBlock) => {
                                                    if !usb_device.poll(&mut [&mut serial]) {
                                                        defmt::warn!(
                                                            "would block: poll returned false"
                                                        );
                                                        break;
                                                    }

                                                    max_retry += 1;
                                                    if max_retry > 3 {
                                                        defmt::warn!("would block: max retry");
                                                        break;
                                                    }
                                                }
                                                Err(_) => {
                                                    defmt::warn!(
                                                        "received an error while sending result"
                                                    );
                                                    break;
                                                }
                                            }
                                        }

                                        if serial.flush().is_err() {
                                            defmt::trace!("cannot flush");
                                        }
                                    }

                                    break;
                                }
                                (None, Some(_)) => {
                                    defmt::warn!("input invalid");
                                    break;
                                }
                                _ => {}
                            }
                        } else {
                            defmt::warn!("invalid utf8 data");
                            break;
                        }
                    }
                }
            }
        }
    }
}

#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [hal::binary_info::EntryAddr; 5] = [
    hal::binary_info::rp_program_name!(c"AoC 2024 day XX"),
    hal::binary_info::rp_cargo_version!(),
    hal::binary_info::rp_program_description!(c"AoC 2024 day XX"),
    hal::binary_info::rp_program_url!(c"private"),
    hal::binary_info::rp_program_build_attribute!(),
];
