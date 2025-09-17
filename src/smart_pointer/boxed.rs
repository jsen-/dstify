extern crate alloc;

use super::{DropGuard, Sealed, SmartPointer};
use alloc::{
    alloc::{Layout, alloc as allocate, handle_alloc_error},
    boxed::Box,
};
use core::ptr;

impl<T: ?Sized> Sealed for Box<T> {}

impl<T: ?Sized> SmartPointer<T> for Box<T> {
    type Guard = DropGuard;

    unsafe fn alloc(layout: Layout) -> (*mut u8, Self::Guard) {
        let base = if layout.size() != 0 {
            let ptr = unsafe { allocate(layout) };
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            ptr
        } else {
            ptr::without_provenance_mut(layout.align())
        };
        (base, DropGuard { base, layout })
    }

    unsafe fn cast(base: *mut T) -> Self {
        unsafe { Box::from_raw(base) }
    }
}
