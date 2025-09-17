extern crate alloc;

use super::{DropGuard, Sealed, SmartPointer};
use alloc::{alloc::Layout, boxed::Box, rc::Rc};

impl<T: ?Sized> Sealed for alloc::rc::Rc<T> {}

impl<T: ?Sized> SmartPointer<T> for Rc<T> {
    type Guard = DropGuard;

    unsafe fn alloc(layout: Layout) -> (*mut u8, Self::Guard) {
        unsafe { Box::<T>::alloc(layout) }
    }

    unsafe fn cast(base: *mut T) -> Self {
        unsafe { Box::from_raw(base) }.into()
    }
}
