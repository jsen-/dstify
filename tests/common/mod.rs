#[macro_export]
macro_rules! make {
    ($ty:path, $($args:expr),+) => {{
        extern crate alloc;
        <$ty>::init_unsized::<alloc::sync::Arc<_>>($($args),*);
        <$ty>::init_unsized::<alloc::rc::Rc<_>>($($args),*);
        <$ty>::init_unsized::<alloc::boxed::Box<_>>($($args),*)
    }};
}
