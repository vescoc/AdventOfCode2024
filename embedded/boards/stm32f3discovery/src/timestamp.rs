use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::hal::{self, prelude::*, timer};
use crate::pac;

use hal::interrupt;

type Instant = fugit::Instant<u64, 1, 1_000_000>;

pub struct Timestamp<TIM: CommonRegisterBlock + timer::Instance> {
    arr: u32,
    timer: UnsafeCell<timer::Timer<TIM>>,
}

impl<TIM: CommonRegisterBlock + timer::Instance> Timestamp<TIM> {
    pub fn new(mut timer: timer::Timer<TIM>) -> Self
    where
        <TIM as hal::interrupts::InterruptNumber>::Interrupt:
            cortex_m::interrupt::InterruptNumber + Copy,
    {
        timer.enable_interrupt(timer::Event::Update);

        timer.start(1.seconds());

        let arr = unsafe { timer.peripheral().get_arr() };

        let interrupt_number = timer.interrupt();
        unsafe {
            pac::NVIC::unmask(interrupt_number);
        }

        Self {
            arr,
            timer: UnsafeCell::new(timer),
        }
    }

    pub fn now(&self) -> Instant {
        let (overflows, counter) = critical_section::with(|_| {
            (
                u64::from(OVERFLOWS.load(Ordering::SeqCst)),
                u64::from(unsafe { (*self.timer.get()).peripheral().get_cnt() }),
            )
        });
        Instant::from_ticks(overflows * 1_000_000 + 1_000_000 * counter / u64::from(self.arr))
    }
}

pub trait CommonRegisterBlock {
    fn get_arr(&self) -> u32;
    fn get_cnt(&self) -> u32;
}

impl CommonRegisterBlock for pac::TIM2 {
    fn get_arr(&self) -> u32 {
        self.arr.read().bits()
    }

    fn get_cnt(&self) -> u32 {
        self.cnt.read().bits()
    }
}

static OVERFLOWS: AtomicU32 = AtomicU32::new(0);
#[interrupt]
fn TIM2() {
    OVERFLOWS.fetch_add(1, Ordering::SeqCst);
    unsafe {
        (*pac::TIM2::ptr()).sr.modify(|_, w| w.uif().clear());
    };
}
