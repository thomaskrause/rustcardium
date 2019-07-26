use super::*;

enum State {
    Opened,
    Closed,
}

pub struct Display {
    state: State,
}

impl Display {
    pub fn open() -> Result<Display> {
        unsafe {
            let result = epic_disp_open();
            if result != 0 {
                return Err(Error::DeviceOrResourceBusy);
            }
        }
        Ok(Display {
            state: State::Opened,
        })
    }

    pub fn close(&mut self) {
        match self.state {
            State::Opened => {
                // alwyays mark the display as closed even if a later error occurs
                self.state = State::Closed;

                unsafe {
                    epic_disp_close();
                }
            }
            State::Closed => {}
        }
    }

    pub fn print(&self, text: &str, fg: Color, bg: Color, posx: u16, posy: u16) -> Result<()> {
        match self.state {
            State::Closed => {
                return Err(Error::DisplayClosed);
            }
            State::Opened => unsafe {
                let result = epic_disp_print(posx, posy, text.as_ptr(), fg.rgb565(), bg.rgb565());
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
