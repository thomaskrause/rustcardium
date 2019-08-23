/// The `buttons` module allows you to use card10’s push buttons as input in your scripts.
/// # Example
/// TODO
///
///
use crate::sys;

///  Bottom left button (bit 0).
pub const LEFT_BOTTOM: u8 = 1;

/// Bottom right button (bit 1).
pub const RIGHT_BOTTOM: u8 = 2;

/// Top right button (bit 2).
pub const RIGHT_TOP: u8 = 4;

/// Top left and reset button (bit 3).
pub const RESET: u8 = 8;

/// Read button status.
///
/// `epic_buttons_read()` will read all buttons specified in mask and return
/// set bits for each button which was reported as pressed.
///
///
/// # Arguments
/// * `mask` - Mask of buttons to read. The 4 LSBs correspond to the 4 buttons:
///
/// | 3     | 2         | 1            | 0           |
/// |-------|-----------|--------------|-------------|
/// | Reset | Right Top | Right Bottom | Left Bottom |
///
/// Use the values defined as constants in this module for masking, as shown in the example.
///
/// # Returns
///
/// Returns nonzero value if unmasked buttons are pushed.
///
/// # Example
/// ```
///    let pressed = rustcardium::buttons::read(rustcardium::buttons::LEFT_BOTTOM | rustcardium::buttons::RIGHT_BOTTOM);
///
///    if (pressed & rustcardium::buttons::LEFT_BOTTOM) > 0 {
///           // Bottom left button is pressed
///    }
///
///    if (pressed & rustcardium::buttons::RIGHT_BOTTOM) > 0 {
///          // Bottom right button is pressed
///    }
/// ```
pub fn read(mask: u8) -> u8 {
    unsafe { sys::epic_buttons_read(mask) }
}
