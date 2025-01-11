//! # USB peripheral.
//!
//! Mostly builds upon the [`stm32_usbd`] crate.
//!
//! ## Examples
//!
//! See [examples/usb_serial.rs] for a usage example.
//!
//! [examples/usb_serial.rs]: https://github.com/stm32-rs/stm32f3xx-hal/blob/v0.10.0/examples/usb_serial.rs

use core::fmt;

#[allow(clippy::module_name_repetitions)]
pub use stm32_usbd::UsbBus;

use crate::hal::pac::USB;
use crate::hal::rcc::{Enable, Reset};

use stm32_usbd::UsbPeripheral;

use crate::hal::gpio;
use crate::hal::gpio::gpioa::{PA11, PA12};

pub trait DmPin {}

pub trait DpPin {}

impl DmPin for PA11<gpio::AF14<gpio::PushPull>> {}

impl DpPin for PA12<gpio::AF14<gpio::PushPull>> {}

/// USB Peripheral
///
/// Constructs the peripheral, which
/// than gets passed to the [`UsbBus`].
#[allow(dead_code)]
pub struct Peripheral<Dm: DmPin, Dp: DpPin> {
    /// USB Register Block
    pub usb: USB,
    /// Data Negativ Pin
    pub pin_dm: Dm,
    /// Data Positiv Pin
    pub pin_dp: Dp,
}

impl<Dm: DmPin + defmt::Format, Dp: DpPin + defmt::Format> defmt::Format for Peripheral<Dm, Dp> {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Peripheral {{ usb: USB, pin_dm: {}, pin_dp: {}}}",
            self.pin_dm,
            self.pin_dp
        );
    }
}

impl<Dm, Dp> fmt::Debug for Peripheral<Dm, Dp>
where
    Dm: DmPin + fmt::Debug,
    Dp: DpPin + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Peripheral")
            .field("usb", &"USB")
            .field("pin_dm", &self.pin_dm)
            .field("pin_dp", &self.pin_dp)
            .finish()
    }
}

// SAFETY: Implementation of Peripheral is thread-safe by using critical sections to ensure
// mutually exclusive access to the USB peripheral
unsafe impl<Dm: DmPin, Dp: DpPin> Sync for Peripheral<Dm, Dp> {}

// SAFETY: The Peirpheral has the same register block layout as STM32 USBFS
unsafe impl<Dm: DmPin + Send, Dp: DpPin + Send> UsbPeripheral for Peripheral<Dm, Dp> {
    const REGISTERS: *const () = USB::ptr().cast::<()>();
    const DP_PULL_UP_FEATURE: bool = false;
    const EP_MEMORY: *const () = 0x4000_6000 as _;
    const EP_MEMORY_SIZE: usize = 512;
    const EP_MEMORY_ACCESS_2X16: bool = false;

    fn enable() {
        // SAFETY: the critical section ensures, that the RCC access to enable the USB peripheral
        // is mutually exclusive
        cortex_m::interrupt::free(|_| unsafe {
            // Enable USB peripheral
            USB::enable_unchecked();
            // Reset USB peripheral
            USB::reset_unchecked();
        });
    }

    fn startup_delay() {
        cortex_m::asm::delay(72);
    }
}

#[allow(dead_code, clippy::module_name_repetitions)]
pub type UsbBusType<Dm = PA11<gpio::AF14<gpio::PushPull>>, Dp = PA12<gpio::AF14<gpio::PushPull>>> =
    UsbBus<Peripheral<Dm, Dp>>;
