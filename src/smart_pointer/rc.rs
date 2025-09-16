extern crate alloc;

use super::{SmartPointer, sealed::Sealed};
use alloc::{alloc::Layout, rc::Rc};
use core::mem;

impl<T: ?Sized> Sealed for alloc::rc::Rc<T> {}

impl<T: ?Sized> SmartPointer<T> for Rc<T> {
    type Guard = Rc<[mem::MaybeUninit<u8>]>;

    fn alloc(layout: Layout) -> (*mut u8, Self::Guard) {
        let arc = Rc::<[u8]>::new_uninit_slice(layout.size());
        let base = Rc::as_ptr(&arc).cast_mut().cast();
        (base, arc)
    }

    unsafe fn cast(base: *mut T) -> Self {
        unsafe { Rc::from_raw(base) }
    }
}
