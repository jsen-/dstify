#![cfg(feature = "std")]

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

    let p0b: Box<_> = P0::init_unsized(Path::new("/usr/bin/true"));
    let p0a: Box<_> = P0::init_unsized(Path::new("/usr/bin/false"));
    assert_ne!(p0a, p0b);

    P1::init_unsized::<Box<_>>(OsStr::new(""));
    P1::init_unsized::<Box<_>>(p0a.path.as_os_str());

    L2::init_unsized::<Box<_>>(&u8::MAX, Path::new("Cargo.toml"), &[]);
    L2::init_unsized::<Box<_>>(&u8::MAX, Path::new("Cargo.toml"), &[1]);
    L2::init_unsized::<Box<_>>(&u8::MAX, Path::new("Cargo.toml"), &[1, 2]);
    L2::init_unsized::<Box<_>>(&u8::MAX, Path::new("Cargo.toml"), &[1, 2, 3]);
}
