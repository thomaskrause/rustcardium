use super::*;

/// The os module allows access to a few core functionalities of Epicardium.

/// Exit from the current app and return to the menu.
///
/// # Arguments
/// * `ret` - Optional return code, same semantics as Posix (0 means success).
///
/// # Returns
///  This function will never return.
pub fn exit(ret: Option<i32>) -> ! {
    unsafe {
        sys::epic_exit(ret.unwrap_or(0));
        unreachable!()
    }
}

/// Stop execution of the current payload and immediately start another payload.
///
/// # Arguments
/// * `name` - Name (path) of the new payload to start. This can either be:
///             * A path to an .elf file (l0dable).
///             * A path to a .py file (will be loaded using Pycardium).
///             * A path to a directory (assumed to be a Python module, execution starts with __init__.py in this folder).
/// # Returns
/// Will only return in case loading went wrong.
pub fn exec(name: &str) -> Result<()> {
    let mut name = create_nullterminated_str(name);
    unsafe {
        let result = sys::epic_exec(name.as_mut_ptr());
        if result >= 0 {
            Ok(())
        } else {
            match (-result) as u32 {
                sys::ENOENT => Err(Error::FileNotFound),
                sys::ENOEXEC => Err(Error::FileNotInLoadableFormat),
                _ => Err(Error::UnknownError),
            }
        }
    }
}
