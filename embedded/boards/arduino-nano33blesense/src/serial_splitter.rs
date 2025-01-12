use core::cell::RefCell;

use crate::hal;

use hal::{timer, uarte, Uarte};

pub fn split<UI: uarte::Instance, TI: timer::Instance>(
    serial: &RefCell<Uarte<UI>>,
    timer: timer::Timer<TI>,
) -> (
    impl embedded_io::Read + use<'_, UI, TI>,
    impl embedded_io::Write + use<'_, UI, TI>,
) {
    (Rx { serial, timer }, Tx { serial })
}

#[derive(Debug)]
pub enum Error {
    Other,
}

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

struct Rx<'a, UI: uarte::Instance, TI: timer::Instance> {
    serial: &'a RefCell<Uarte<UI>>,
    timer: timer::Timer<TI>,
}

impl<UI: uarte::Instance, TI: timer::Instance> embedded_io::ErrorType for Rx<'_, UI, TI> {
    type Error = Error;
}

impl<UI: uarte::Instance, TI: timer::Instance> embedded_io::Read for Rx<'_, UI, TI> {
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        let mut serial = self.serial.borrow_mut();
        let mut len = 0;
        for b in buffer.chunks_mut(255) {
            loop {
                match serial.read_timeout(b, &mut self.timer, 1_000_000) {
                    Ok(()) => {
                        len += b.len();
                        break;
                    }
                    Err(uarte::Error::Timeout(size)) => {
                        if len + size > 0 {
                            return Ok(len + size);
                        }
                    }
                    Err(_) => return Err(Error::Other),
                }
            }
        }
        Ok(len)
    }
}

struct Tx<'a, UI: uarte::Instance> {
    serial: &'a RefCell<Uarte<UI>>,
}

impl<UI: uarte::Instance> embedded_io::ErrorType for Tx<'_, UI> {
    type Error = Error;
}

impl<UI: uarte::Instance> embedded_io::Write for Tx<'_, UI> {
    fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error> {
        let mut buf = [0u8; 255];
        let buffer = if 0x2000_0000 > buffer.as_ptr() as usize {
            assert!(buffer.len() < 255, "buffer too big");

            unsafe {
                buffer.as_ptr().copy_to(buf.as_mut_ptr(), buffer.len());
            }

            &buf[0..buffer.len()]
        } else {
            buffer
        };

        self.serial
            .borrow_mut()
            .write(buffer)
            .map_err(|_| Error::Other)?;

        Ok(buffer.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
