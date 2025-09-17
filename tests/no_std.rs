mod common;

use core::ffi::CStr;
use dstify::Dstify;

#[derive(Dstify)]
#[repr(C)]
struct N0 {
    dst: [u8],
}

#[derive(Dstify, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
struct N1 {
    a1: u8,
    dst: [u8],
}

#[derive(Dstify)]
#[repr(C)]
struct N2 {
    a1: u8,
    a2: u16,
    dst: [u16],
}

#[derive(Dstify)]
#[repr(C)]
struct N3 {
    a1: u8,
    a2: u16,
    a3: u32,
    dst: [()],
}

#[derive(Dstify)]
#[repr(C)]
struct N4 {
    a1: (),
    a2: (),
    a3: (),
    a4: (),
    dst: [u128],
}

#[derive(Dstify)]
#[repr(C)]
struct N5 {
    dst: str,
}

#[derive(Dstify)]
#[repr(C)]
struct N6 {
    dst: CStr,
}

#[derive(Dstify)]
#[repr(C)]
struct U0([u8]);

#[derive(Dstify)]
#[repr(C)]
struct U1(u8, [u8]);

#[derive(Dstify)]
#[repr(C)]
struct Z0([()]);

#[derive(Dstify)]
#[repr(C)]
struct Z1((), [()]);

#[derive(Dstify)]
#[repr(C)]
struct Z2((), (), [()]);

#[derive(Dstify)]
#[repr(C)]
struct L1<'a>(&'a u8, [u8]);

#[test]
fn test() {
    assert_eq!(size_of::<&N0>(), 16);
    assert_eq!(size_of::<&N1>(), 16);
    assert_eq!(size_of::<&N2>(), 16);
    assert_eq!(size_of::<&N3>(), 16);
    assert_eq!(size_of::<&N4>(), 16);
    assert_eq!(size_of::<&N5>(), 16);
    assert_eq!(size_of::<&N6>(), 16);
    assert_eq!(size_of::<&U0>(), 16);
    assert_eq!(size_of::<&U1>(), 16);
    assert_eq!(size_of::<&L1>(), 16);

    make!(N0, &[]);
    make!(N0, &[1]);
    make!(N0, &[1, 2]);
    make!(N0, &[1, 2, 3]);

    let t1a = make!(N1, 0, &[]);
    let t1b = make!(N1, 0, &[]);
    assert_eq!(t1a, t1b);

    let t1c = N1::init_unsized::<Box<_>>(1, &[1]);
    let t1d = N1::init_unsized::<Box<_>>(2, &[1, 2]);
    assert!(t1c < t1d);

    assert!(N1::init_unsized_checked::<Box<_>>(0, &[0, 1, 2, 3, 4]).is_ok());
    #[cfg(not(miri))]
    assert!(
        N1::init_unsized_checked::<Box<_>>(0, unsafe {
            &*core::ptr::slice_from_raw_parts(core::ptr::dangling(), isize::MAX as usize)
        })
        .is_err()
    );

    make!(N3, 1, 2, 3, &[]);
    make!(N3, 1, 2, 3, &[()]);
    make!(N3, 1, 2, 3, &[(), ()]);

    make!(N4, (), (), (), (), &[]);
    make!(N4, (), (), (), (), &[1]);
    make!(N4, (), (), (), (), &[1, 2]);

    make!(N5, "");
    make!(N5, "Hello, World!");

    make!(N6, CStr::from_bytes_with_nul(b"\0").unwrap());
    make!(N6, CStr::from_bytes_with_nul(b"Hello, World!\0").unwrap());

    make!(U0, &[1]);
    make!(U1, 1, &[1]);

    make!(Z0, &[]);
    make!(Z0, &[()]);
    make!(Z0, &[(), ()]);

    make!(Z1, (), &[]);
    make!(Z1, (), &[()]);
    make!(Z1, (), &[(), ()]);

    make!(Z2, (), (), &[]);
    make!(Z2, (), (), &[()]);
    make!(Z2, (), (), &[(), ()]);

    make!(L1, &1, &[]);
    make!(L1, &1, &[1]);
    make!(L1, &1, &[1, 2]);
    make!(L1, &1, &[1, 2]);
}
