extern crate winapi;

#[macro_use]
extern crate clap;

use clap::App;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::c_void;
use std::io::Error;
use std::os::windows::prelude::*;
use std::path::Path;

use self::winapi::shared::minwindef::{TRUE, FALSE};
use winapi::um::winnt::{PVOID, WCHAR};
use winapi::um::winuser::{SystemParametersInfoW, SPI_GETDESKWALLPAPER};
use winapi::um::winuser::SPIF_SENDCHANGE;
use winapi::um::winuser::SPIF_UPDATEINIFILE;
use winapi::um::winuser::SPI_SETDESKWALLPAPER;
use std::ffi::{OsString, OsStr};

const MAXPATH : usize = winapi::shared::minwindef::MAX_PATH;
type Res<T> = std::result::Result<T, std::io::Error>;

fn set_wallpaper(path: &str) -> Res<&str> {
    let mut wpath: Vec<u16> = OsStr::new(path).encode_wide().chain(once(0)).collect::<Vec<_>>();
    let ret = unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            wpath.as_mut_ptr() as PVOID,
            SPIF_SENDCHANGE | SPIF_UPDATEINIFILE,
        )
    };
    match ret {
        TRUE => Ok("YAY! 102 Successfully changed wallpaper."),
        _ => Err(Error::last_os_error().into()),
    }
}

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
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("get") {
        println!("{:?}", (get_wallpaper()).unwrap());
    } else if let Some(matches) = matches.subcommand_matches("set") {
        if (Path::new((matches.value_of("FILE").unwrap())).exists()) {
            println!("VERB 100 Attempting to set image to path {}", matches.value_of("FILE").unwrap());
            println!("{:#?}", (set_wallpaper(matches.value_of("FILE").unwrap())).unwrap());
        } else {
            println!("ERR! 101 The file {} does not exist. Fatal.", matches.value_of("FILE").unwrap())
        }
    } else {
        println!("ERR! 001 No valid flag, value, subcommand, or other argument was recieved. Fatal.")
    }
}
