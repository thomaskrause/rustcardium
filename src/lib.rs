#![no_std]

mod display;

pub use display::Display;

type CInt = i32;

#[link(name="api-caller", kind="static")]
#[link(name="PeriphDriver", kind="static")]
extern "C" {
    fn epic_disp_open() -> CInt;
    fn epic_disp_close() -> CInt;
    fn epic_disp_clear(color : u16) -> CInt;
    fn epic_disp_print(posx : u16, posy : u16, pString :  *const u8, fg : u16, bg : u16) -> CInt;
    fn epic_disp_pixel(x : u16, y : u16, color : u16) -> CInt;
    fn epic_disp_update() -> CInt;
}

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


/// Custom error variants for Epicardium.
#[derive(Debug)]
pub enum Error {
    DisplayClosed,
    OutsideDisplay,
    DeviceOrResourceBusy
}

pub type Result<T> = core::result::Result<T, Error>;

