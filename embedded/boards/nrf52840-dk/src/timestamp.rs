use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::hal::pac;
use crate::hal::timer;

use pac::interrupt;

type Instant = fugit::Instant<u64, 1, 1_000_000>;

pub struct Timestamp(());

impl Timestamp {
    pub fn new(mut timer: timer::Timer<pac::TIMER2, timer::Periodic>) -> Self {
        timer.enable_interrupt();

        let interrupt_number = <pac::TIMER2 as timer::Instance>::INTERRUPT;

        timer.start(1_000_000);

        critical_section::with(|cs| TIMER.replace(cs, Some(timer)));

        unsafe {
            pac::NVIC::unmask(interrupt_number);
        }

        Self(())
    }

    #[allow(clippy::unused_self)]
    pub fn now(&self) -> Instant {
        let (overflows, counter) = critical_section::with(|cs| {
            (
                u64::from(OVERFLOWS.load(Ordering::SeqCst)),
                u64::from(TIMER.borrow_ref(cs).as_ref().unwrap().read()),
            )
        });

        Instant::from_ticks(overflows * 1_000_000 + counter)
    }
}

static OVERFLOWS: AtomicU32 = AtomicU32::new(0);
static TIMER: critical_section::Mutex<RefCell<Option<timer::Timer<pac::TIMER2, timer::Periodic>>>> =
    critical_section::Mutex::new(RefCell::new(None));

#[interrupt]
fn TIMER2() {
    OVERFLOWS.fetch_add(1, Ordering::SeqCst);
    critical_section::with(|cs| {
        TIMER.borrow_ref(cs).as_ref().unwrap().reset_event();
    });
}
