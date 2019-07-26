use super::*;

pub fn open() -> Result<()> {
    unsafe {
        let result = epic_disp_open();
        if result != 0 {
            return Err(Error::DeviceOrResourceBusy);
        }
    }
    Ok(())
}

pub fn close() -> Result<()> {
    unsafe {
        let result = epic_disp_close();
        if result != 0 {
            return Err(Error::DeviceOrResourceBusy);
        }
    }
    Ok(())
}

pub fn print(text: &str, fg : Color, bg : Color, posx : u16, posy : u16) -> Result<()> {
    unsafe {
        let result = epic_disp_print(posx, posy, text.as_ptr(), fg.rgb565(), bg.rgb565());
        if result != 0 {
            return Err(Error::DeviceOrResourceBusy);
        }
    }
    Ok(())
}