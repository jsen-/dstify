#![cfg(feature = "std")]

mod common;

use dstify::Dstify;
use std::{ffi::OsStr, path::Path};

#[derive(Dstify, Debug, PartialEq, Eq)]
#[repr(C)]
struct P0 {
    path: Path,
}

#[derive(Dstify)]
#[repr(C)]
struct P1 {
    s: OsStr,
}

#[derive(Dstify)]
#[repr(C)]
struct L2<'a, 'b>(&'a u8, &'b Path, [u8]);

#[test]
fn test() {
    assert_eq!(size_of::<&P0>(), 16);
    assert_eq!(size_of::<&P1>(), 16);

    let p0b = make!(P0, Path::new("/usr/bin/true"));
    let p0a = make!(P0, Path::new("/usr/bin/false"));
    assert_ne!(p0a, p0b);

    make!(P1, OsStr::new(""));
    make!(P1, p0a.path.as_os_str());

    make!(L2, &u8::MAX, Path::new("Cargo.toml"), &[]);
    make!(L2, &u8::MAX, Path::new("Cargo.toml"), &[1]);
    make!(L2, &u8::MAX, Path::new("Cargo.toml"), &[1, 2]);
    make!(L2, &u8::MAX, Path::new("Cargo.toml"), &[1, 2, 3]);
}
