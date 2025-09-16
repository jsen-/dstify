extern crate alloc;

mod arc;
mod boxed;
mod rc;

use alloc::alloc::Layout;

mod sealed {
    pub trait Sealed {}
}

/// Internal trait implemented for smart pointer types that `init_unsized` and `init_unsized_checked` can return.
pub trait SmartPointer<T: ?Sized>: sealed::Sealed {
    type Guard;
    fn alloc(layout: Layout) -> (*mut u8, Self::Guard);

    #[allow(clippy::missing_safety_doc)]
    unsafe fn cast(base: *mut T) -> Self;
}
