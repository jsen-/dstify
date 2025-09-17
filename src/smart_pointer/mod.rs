mod arc;
mod boxed;
mod rc;

use alloc::alloc::{Layout, dealloc};

pub trait Sealed {}

/// Internal trait implemented for smart pointer types that `init_unsized` and `init_unsized_checked` can return.
#[allow(clippy::missing_safety_doc)]
pub trait SmartPointer<T: ?Sized>: Sealed {
    type Guard;

    unsafe fn alloc(layout: Layout) -> (*mut u8, Self::Guard);

    unsafe fn cast(base: *mut T) -> Self;
}

pub struct DropGuard {
    base: *mut u8,
    layout: Layout,
}
impl Drop for DropGuard {
    fn drop(&mut self) {
        if self.layout.size() != 0 {
            unsafe { dealloc(self.base, self.layout) };
        }
    }
}
