extern crate winapi;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::prelude::*;
use self::winapi::shared::minwindef::{TRUE, FALSE};
use winapi::um::winnt::{PVOID, WCHAR};
use winapi::um::winuser::{SystemParametersInfoW, SPI_GETDESKWALLPAPER};
use winapi::um::winuser::SPIF_SENDCHANGE;
use winapi::um::winuser::SPIF_UPDATEINIFILE;
use winapi::um::winuser::SPI_SETDESKWALLPAPER;
use std::ffi::{OsString, OsStr};
const MAXPATH : usize = winapi::shared::minwindef::MAX_PATH;
#[cfg(windows)]
fn get_wallpaper() -> Res<OsString> { // winapi::ctypes::c_void {
    // let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let imgpath = [0 as WCHAR; MAXPATH];
    let ret = unsafe {
        SystemParametersInfoW(
            SPI_GETDESKWALLPAPER,
            MAXPATH as u32,
            imgpath.as_ptr() as PVOID,
            0
        )
    };
    let strpos = imgpath.iter().position(|&a| a == 0).unwrap();
    let path = OsString::from_wide(&imgpath[..strpos]);
    match ret {
        TRUE => Ok(path),
        _ => Err(Error::last_os_error().into()),
    }
}

fn main() {
    let yaml = load_yaml!("args.yml");
        println!("{:?}", (get_wallpaper()).unwrap());
}
