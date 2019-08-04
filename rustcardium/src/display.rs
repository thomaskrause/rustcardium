use super::*;

enum State {
    Opened,
    Closed,
}

/// The display class provides methods to allow the lcd display
/// in card10 to be used in a safe way.
///
/// You can close the display manually, or it will automatically be closed when the variable gets out of scope.
///
/// # Example
/// ```
/// if let Ok(display) = epicardium::Display::open() {
///     let color_black = epicardium::Color {r: 0, g: 0, b: 0};
///     let color_white = epicardium::Color {r: 255, g: 255, b: 255};
///
///     display.print("Hello World", 0, 0, black, white).unwrap();
/// }
/// ```
pub struct Display {
    state: State,
}

impl Display {
    /// Opens the display. Will fail if the display can't be locked
    pub fn open() -> Result<Display> {
        unsafe {
            let result = epicardium_sys::epic_disp_open();
            if result != 0 {
                return Err(Error::DeviceOrResourceBusy);
            }
        }
        Ok(Display {
            state: State::Opened,
        })
    }

    /// Closes and unlocks the display.
    /// To be able to use it again, it is necessary to open and lock it again with Display.open()
    pub fn close(&mut self) {
        match self.state {
            State::Opened => {
                // alwyays mark the display as closed even if a later error occurs
                self.state = State::Closed;

                unsafe {
                    epicardium_sys::epic_disp_close();
                }
            }
            State::Closed => {}
        }
    }

    /// Updates the display based on the changes previously made by various draw functions
    pub fn update(&self) -> Result<()> {
        match self.state {
            State::Closed => {
                return Err(Error::DisplayClosed);
            }
            State::Opened => unsafe {
                let result = epicardium_sys::epic_disp_update();
                if result != 0 {
                    return Err(Error::DeviceOrResourceBusy);
                }
            },
        }
        Ok(())
    }

    /// Clears the display using the color provided, or the default color black.
    ///
    /// `col` - Clearing color
    pub fn clear(&self, col: Option<Color>) -> Result<()> {
        match self.state {
            State::Closed => {
                return Err(Error::DisplayClosed);
            }
            State::Opened => unsafe {
                let result = epicardium_sys::epic_disp_clear(col.unwrap_or(Color { r: 0, g: 0, b: 0 }).rgb565());
                if result != 0 {
                    return Err(Error::DeviceOrResourceBusy);
                }
            },
        }
        Ok(())
    }

    /// Prints a string on the display. Font size is locked to 20px
    ///
    /// - `text` - Text to print
    /// - `fg` - Foreground color
    /// - `bg` - Background color
    /// - `posx` - X-Position of the first character, 0 <= posx <= 160
    /// - `posy` - Y-Position of the first character, 0 <= posy <= 80
    pub fn print(&self, text: &str, fg: Color, bg: Color, posx: u16, posy: u16) -> Result<()> {
        match self.state {
            State::Closed => {
                return Err(Error::DisplayClosed);
            }
            State::Opened => unsafe {
                let text = create_nullterminated_str(text);
                let result = epicardium_sys::epic_disp_print(posx, posy, text.as_ptr() as *const i8, fg.rgb565(), bg.rgb565());
                if result != 0 {
                    return Err(Error::DeviceOrResourceBusy);
                }
            },
        }
        Ok(())
    }

    /// Draws a pixel on the display
    ///
    /// `x` - X coordinate, 0<= x <= 160
    /// `y` - Y coordinate, 0<= y <= 80
    /// `col` - color of the pixel
    pub fn pixel(&self, x: u16, y: u16, col : Color) -> Result<()> {
        if x > 160 || y > 80 {
            return Err(Error::OutsideDisplay);
        }

        match self.state {
            State::Closed => {
                return Err(Error::DisplayClosed);
            }
            State::Opened => unsafe {
                let result = epicardium_sys::epic_disp_pixel(x, y, col.rgb565());
                if result != 0 {
                    return Err(Error::DeviceOrResourceBusy);
                }
            },
        }
        Ok(())
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        self.close();
    }
}
