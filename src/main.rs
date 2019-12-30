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
    let mut wpath: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(once(0))
        .collect::<Vec<_>>();
    let ret = unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            wpath.as_mut_ptr() as PVOID,
            SPIF_SENDCHANGE | SPIF_UPDATEINIFILE,
        )
    };
    match ret {
        TRUE => Ok(""),
        _ => Err(Error::last_os_error().into()),
    }
}

#[cfg(windows)]
fn get_wallpaper() -> Res<OsString> {
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
        println!("{}", (get_wallpaper()).unwrap());
    } else if let Some(matches) = matches.subcommand_matches("set") {
        let file = matches.value_of("FILE").unwrap();
        if fs::metadata(&*file).is_ok() {
            let ret = set_wallpaper(matches.value_of("FILE").unwrap()));
            match ret {
                Ok(_)  => (),
                Err(e) => print!("painter: error: {}", e),
            }
        } else {
            println!("painter: error: provided path '{}' doesn't exist.", file);
        }
    } else {
        ()
    }
}
