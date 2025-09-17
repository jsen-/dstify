extern crate alloc;

use crate::SmartPointer;
use core::{
    alloc::{Layout, LayoutError},
    ffi::CStr,
    mem, ptr,
};

#[cfg(feature = "std")]
use std::{ffi::OsStr, path::Path};

pub unsafe fn alloc<T, R, D, F, const N: usize>(
    normal_fields: [core::alloc::Layout; N],
    unsized_field: &D,
    init_normal_fields: F,
) -> Result<*const [u8], core::alloc::LayoutError>
where
    T: ?Sized,
    R: SmartPointer<T>,
    D: AsSlice + ?Sized,
    F: FnOnce(&mut Offsets<N>),
{
    let slice = unsized_field.as_slice();
    let (layout, offsets, last_offset) = calc_offsets(normal_fields, slice)?;
    let (base, guard) = unsafe { R::alloc(layout) };

    let mut offsets = Offsets {
        base,
        offsets,
        curr: 0,
    };
    init_normal_fields(&mut offsets);
    if !slice.is_empty() {
        unsafe {
            let dest = base.add(last_offset).cast();
            ptr::copy_nonoverlapping(slice.as_ptr(), dest, slice.len())
        }
    }

    mem::forget(guard);
    Ok(::core::ptr::slice_from_raw_parts_mut(base, slice.len()))
}

#[inline]
fn calc_offsets<const N: usize, T>(
    normal_fields: [Layout; N],
    slice: &[T],
) -> Result<(Layout, [usize; N], usize), LayoutError> {
    let mut offsets: [usize; N] = [0; N];
    let (layout, last_offset) = if N != 0 {
        let mut layout = normal_fields[0];
        for i in 1..N {
            let (new_layout, offset) = layout.extend(normal_fields[i])?;
            layout = new_layout;
            offsets[i] = offset;
        }
        layout.extend(Layout::array::<T>(slice.len())?)?
    } else {
        (Layout::array::<T>(slice.len())?, 0)
    };
    Ok((layout.pad_to_align(), offsets, last_offset))
}

pub struct Offsets<const N: usize> {
    pub(super) base: *mut u8,
    pub(super) offsets: [usize; N],
    pub(super) curr: usize,
}

impl<const N: usize> Offsets<N> {
    #[inline]
    pub fn get_next(&mut self) -> *mut u8 {
        assert!(self.curr < N);
        let ret = unsafe { self.base.add(self.offsets[self.curr]) };
        self.curr += 1;
        ret
    }
    #[inline]
    pub fn base(&self) -> *mut u8 {
        self.base
    }
}

mod sealed {
    pub trait Sealed {}
}

pub trait AsSlice: sealed::Sealed {
    type Item: Copy;
    fn as_slice(&self) -> &[Self::Item];
}

// bitwise copy of the resulting slice is performed so `T` must be `Copy`
impl<T> sealed::Sealed for [T] where T: Copy {}
impl<T> AsSlice for [T]
where
    T: Copy,
{
    type Item = T;
    fn as_slice(&self) -> &[Self::Item] {
        self
    }
}

impl sealed::Sealed for str {}
impl AsSlice for str {
    type Item = u8;
    fn as_slice(&self) -> &[Self::Item] {
        self.as_bytes()
    }
}

impl sealed::Sealed for CStr {}
impl AsSlice for CStr {
    type Item = u8;
    fn as_slice(&self) -> &[Self::Item] {
        self.to_bytes_with_nul()
    }
}

#[cfg(feature = "std")]
impl sealed::Sealed for OsStr {}
#[cfg(feature = "std")]
impl AsSlice for OsStr {
    type Item = u8;
    fn as_slice(&self) -> &[Self::Item] {
        self.as_encoded_bytes()
    }
}

#[cfg(feature = "std")]
impl sealed::Sealed for Path {}
#[cfg(feature = "std")]
impl AsSlice for Path {
    type Item = u8;
    fn as_slice(&self) -> &[Self::Item] {
        self.as_os_str().as_encoded_bytes()
    }
}
