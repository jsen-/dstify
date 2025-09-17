use super::{DropGuard, Sealed, SmartPointer};
use alloc::{alloc::Layout, boxed::Box, sync::Arc};

impl<T: ?Sized> Sealed for alloc::sync::Arc<T> {}

impl<T: ?Sized> SmartPointer<T> for Arc<T> {
    type Guard = DropGuard;

    unsafe fn alloc(layout: Layout) -> (*mut u8, Self::Guard) {
        unsafe { Box::<T>::alloc(layout) }
    }

    unsafe fn cast(base: *mut T) -> Self {
        unsafe { Box::from_raw(base) }.into()
    }
}
