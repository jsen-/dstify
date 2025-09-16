extern crate alloc;

use super::{SmartPointer, sealed::Sealed};
use alloc::alloc::{Layout, alloc as allocate, dealloc, handle_alloc_error};
use core::ptr;

impl<T: ?Sized> Sealed for alloc::boxed::Box<T> {}

impl<T: ?Sized> SmartPointer<T> for Box<T> {
    type Guard = DropGuard;

    fn alloc(layout: Layout) -> (*mut u8, Self::Guard) {
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
