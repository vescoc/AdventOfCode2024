#![no_std]

use core::cell::RefCell;

use embedded_io::{ErrorType, Read, Write};

use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

struct SplitterInner<'b, B: UsbBus> {
    usb_device: UsbDevice<'b, B>,
    serial_port: SerialPort<'b, B>,
}

impl<B: UsbBus> SplitterInner<'_, B> {
    fn poll(&mut self) {        
        while !self.usb_device.poll(&mut [&mut self.serial_port]) {}
    }
}

pub struct Splitter<'b, B: UsbBus>(RefCell<SplitterInner<'b, B>>);

impl<'b, B: UsbBus> Splitter<'b, B> {
    pub fn new(usb_device: UsbDevice<'b, B>, serial_port: SerialPort<'b, B>) -> Self {
        Self(RefCell::new(SplitterInner { usb_device, serial_port }))
    }
    
    pub fn split<'a>(&'a self) -> (WrapperRx<'a, 'b, B>, WrapperTx<'a, 'b, B>) {
        (WrapperRx(self), WrapperTx(self))
    }
}

pub struct WrapperTx<'a, 'b: 'a, B: UsbBus>(&'a Splitter<'b, B>);
pub struct WrapperRx<'a, 'b: 'a, B: UsbBus>(&'a Splitter<'b, B>);

impl<'a, 'b: 'a, B: UsbBus> ErrorType for WrapperTx<'a, 'b, B> {
    type Error = <SerialPort<'b, B> as ErrorType>::Error;
}

impl<B: UsbBus> Write for WrapperTx<'_, '_, B> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        Ok(self.0.0.borrow_mut().serial_port.write(buf)?)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(self.0.0.borrow_mut().serial_port.flush()?)
    }
}

impl<'a, 'b: 'a, B: UsbBus> ErrorType for WrapperRx<'a, 'b, B> {
    type Error = <SerialPort<'b, B> as ErrorType>::Error;
}

impl<B: UsbBus> Read for WrapperRx<'_, '_, B> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut this = self.0.0.borrow_mut();

        this.poll();

        Ok(this.serial_port.read(buf)?)
    }
}
