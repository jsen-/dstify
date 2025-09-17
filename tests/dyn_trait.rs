extern crate alloc;
use alloc::{boxed::Box, rc::Rc, sync::Arc};
use core::net::IpAddr;
use dstify::Dstify;

#[derive(Dstify)]
#[repr(C)]
struct D0Debug {
    dbg: dyn std::fmt::Debug,
}

#[derive(Dstify)]
#[repr(C)]
struct D0DebugSend {
    dst: dyn std::fmt::Debug + Send,
}

#[derive(Dstify)]
#[repr(C)]
struct D0DebugSendSync {
    dst: dyn std::fmt::Debug + Send + Sync,
}

#[derive(Dstify)]
#[repr(C)]
struct D1Debug {
    a: u8,
    dst: dyn std::fmt::Debug,
}

#[derive(Dstify)]
#[repr(C)]
struct D1DebugSend {
    a: u8,
    dst: dyn std::fmt::Debug + Send,
}

#[derive(Dstify)]
#[repr(C)]
struct D0Display {
    disp: dyn std::fmt::Display,
}

#[derive(Dstify)]
#[repr(C)]
struct U0Debug(dyn std::fmt::Debug);

macro_rules! test_debug {
    ($ty:path => $field:ident [ $($init:expr),* ] $val:expr, $($rest:expr),+) => {
        test_debug!($ty => $field [$($init),*] $val);
        test_debug!($ty => $field [$($init),*] $($rest),*);
    };
    ($ty:path => $field:ident [ $($init:expr),* ] $val:expr) => {{
        let x = <$ty>::init_unsized::<Arc<_>, _>($($init,)* $val);
        assert_eq!(format!("{:?}", &x.$field), format!("{:?}", $val));
        let x = <$ty>::init_unsized::<Rc<_>, _>($($init,)* $val);
        assert_eq!(format!("{:?}", &x.$field), format!("{:?}", $val));
        let x = <$ty>::init_unsized::<Box<_>, _>($($init,)* $val);
        assert_eq!(format!("{:?}", &x.$field), format!("{:?}", $val));
    }};
}

macro_rules! test_display {
    ($ty:path => $field:ident [ $($init:expr),* ] $val:expr, $($rest:expr),+) => {
        test_display!($ty => $field [$($init),*] $val);
        test_display!($ty => $field [$($init),*] $($rest),*);
    };
    ($ty:path => $field:ident [ $($init:expr),* ] $val:expr) => {{
        let x = <$ty>::init_unsized::<Arc<_>, _>($($init,)* $val);
        assert_eq!(format!("{}", &x.$field), format!("{}", $val));
        let x = <$ty>::init_unsized::<Rc<_>, _>($($init,)* $val);
        assert_eq!(format!("{}", &x.$field), format!("{}", $val));
        let x = <$ty>::init_unsized::<Box<_>, _>($($init,)* $val);
        assert_eq!(format!("{}", &x.$field), format!("{}", $val));
    }};
}

trait X {
    fn x(&self) -> &'static str;
}
impl X for () {
    fn x(&self) -> &'static str {
        "()"
    }
}
impl X for u64 {
    fn x(&self) -> &'static str {
        "u64"
    }
}

#[derive(Dstify)]
#[repr(C)]
struct WithX(u8, dyn X);

#[test]
fn test() {
    test_debug!(D0Debug => dbg [] u64::MAX, 14.4f32, "Hello, World!", String::from("test"), b"blob", (), true);
    test_debug!(D0DebugSend => dst [] u64::MAX, 14.4f32, "Hello, World!", (), true);
    test_debug!(D0DebugSendSync => dst [] u64::MAX, 14.4f32, "Hello, World!", (), true);

    test_debug!(D1Debug => dst [10] u64::MAX, 14.4f32, "Hello, World!", (), true);
    test_debug!(D1DebugSend => dst [20] u64::MAX, 14.4f32, "Hello, World!", (), true);

    test_display!(D0Display => disp [] u64::MAX, 10isize, 14.4f32, "Hello, World!", "127.0.0.1".parse::<IpAddr>().unwrap());

    let val = String::from("a test");
    let x: Rc<_> = U0Debug::init_unsized_checked(val.clone()).unwrap();
    assert_eq!(format!("{:?}", &x.0), format!("{val:?}"));

    let x: Box<_> = WithX::init_unsized(1, ());
    assert_eq!(x.1.x(), ().x());

    let x: Arc<_> = WithX::init_unsized(10, 10u64);
    assert_eq!(x.1.x(), 10.x());
}
