use crate::sys::*;
use core::fmt::Write;

pub struct Uart;

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            epic_uart_write_str(s.as_ptr(), s.len() as isize);
        }
        Ok(())
    }
}
