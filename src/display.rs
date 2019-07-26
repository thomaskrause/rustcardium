use super::*;

pub fn print(posx : u16, posy : u16, p_string :  &str, fg : Color, bg : Color) -> Result<()> {
    unsafe {
        let result = epic_disp_print(posx, posy, p_string.as_ptr(), fg.rgb565(), bg.rgb565());
        if result != 0 {
            return Err(Error::DeviceOrResourceBusy);
        }
    }
    Ok(())
}