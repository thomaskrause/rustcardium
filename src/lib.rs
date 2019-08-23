#![no_std]
#![feature(global_asm)]

pub mod buttons;
pub mod display;
pub mod os;
mod sys;
pub mod uart;

use arrayvec::ArrayString;

use panic_abort as _;

global_asm!(include_str!("crt.s"));

#[macro_export]
macro_rules! main {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() {
            // type check the given path
            let f: fn() = $path;

            f()
        }
    };
}

#[link_section = ".text.boot"]
#[no_mangle]
pub unsafe extern "C" fn Reset_Handler() -> ! {
    extern "C" {
        fn SystemInit();
        // Boundaries of the .bss section, provided by the linker script
        static mut __bss_start: u64;
        static mut __bss_end: u64;
    }

    // Zeroes the .bss section
    r0::zero_bss(&mut __bss_start, &mut __bss_end);
    SystemInit();

    extern "Rust" {
        fn main();
    }

    main();
    os::exit(Some(0));
}

pub const UART: uart::Uart = uart::Uart;

/// Representation of a RGB color value.
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    fn rgb565(self) -> u16 {
        let raw = ((u16::from(self.r) & 0xF8) << 8)
            | ((u16::from(self.g) & 0xFA) << 3)
            | (u16::from(self.b) & 0xF8);
        raw
    }
}

fn create_nullterminated_str(text: &str) -> ArrayString<[u8; 1024]> {
    if text.len() < 1024 && text.ends_with("\0") {
        return ArrayString::from(text).unwrap();
    } else {
        let mut new_text = ArrayString::from(&text[0..core::cmp::min(1023, text.len())]).unwrap();
        new_text.push('\0');
        return new_text;
    }
}

/// Custom error variants for Epicardium.
#[derive(Debug)]
pub enum Error {
    DisplayClosed,
    OutsideDisplay,
    DeviceOrResourceBusy,
    FileNotFound,
    FileNotInLoadableFormat,
    UnknownError,
}

pub type Result<T> = core::result::Result<T, Error>;
