extern crate alloc;

use super::{SmartPointer, sealed::Sealed};
use alloc::{alloc::Layout, sync::Arc};
use core::mem;

impl<T: ?Sized> Sealed for alloc::sync::Arc<T> {}

impl<T: ?Sized> SmartPointer<T> for Arc<T> {
    type Guard = Arc<[mem::MaybeUninit<u8>]>;

    fn alloc(layout: Layout) -> (*mut u8, Self::Guard) {
        let arc = Arc::<[u8]>::new_uninit_slice(layout.size());
        let base = Arc::as_ptr(&arc).cast_mut().cast();
        (base, arc)
    }

    unsafe fn cast(base: *mut T) -> Self {
        unsafe { Arc::from_raw(base) }
    }
}
