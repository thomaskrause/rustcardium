#![no_std]

pub mod buttons;
pub mod display;
pub mod os;

use arrayvec::ArrayString;

/// Representation of a RGB color value.
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    fn rgb565(self) -> u16 {
        let r5 = ((self.r as u16) >> 3) as u8;
        let g6 = ((self.g as u16) >> 2) as u8;
        let b5 = ((self.b as u16) >> 3) as u8;

        let result1 = ((g6 & 0b000111) << 5) | b5;
        let result2 = (r5 << 3) | ((g6 & 0b111000) >> 3);

        ((result1 as u16) << 8) | (result2 as u16)
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
