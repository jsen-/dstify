#![cfg_attr(not(feature = "std"), no_std)]

//! # DSTify
//!
//! This crate enables safe construction of custom dynamically-sized types (DSTs).
//!
//! Consists of a `derive` procedural macro that can be applied on any `#[repr(C)]` struct with dynamically-sized last field.
//! Structs with both named and unnamed fields (tuple structs) are supported. `dyn Trait` last fields, however, are not (yet).
//!
//! ```no_run
//! # #[cfg(feature = "std")]
//! # {
//! # use std::fs::File;
//! # use std::path::Path;
//! // example
//! use dstify::Dstify;
//!
//! #[derive(Dstify, Debug)]
//! #[repr(C)]
//! struct FileWithPath {
//!     file: File,
//!     path: Path,
//! }
//!
//! impl FileWithPath {
//!     pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Box<Self>> {
//!         let path = path.as_ref();
//!         let file = File::open(path)?;
//!         Ok(FileWithPath::init_unsized(file, path))
//!     }
//!     pub fn path(&self) -> &Path {
//!         &self.path
//!     }
//!     pub fn file(&self) -> &File {
//!         &self.file
//!     }
//! }
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!
//! let named_file = FileWithPath::open("Cargo.toml")?;
//! println!("{named_file:?}");
//! assert_eq!(size_of_val(&named_file), 16);
//! # Ok(())
//! # }
//! # }
//! ```
//! The `Dstify` proc macro in above example generates an impl block for the `FileWithPath` struct with two "static" methods, `init_unsized` and `init_unsized_checked`.
//! Both of them accept all the struct fields as arguments in definition order. The type of the last one, being a DST, becomes a reference.
//!
//! The return type `R`, determines the smart pointer type that should be constructed. The bounding trait - [`SmartPointer`], is implemented for [`Box`], [`Rc`](`std::rc::Rc`) and [`Arc`](`std::sync::Arc`).
//!
//! The `checked` method returns `LayoutError` if the size of the resulting instance would exceed `isize::MAX` bytes.
//! The "unchecked" method panics in that case.
//!
//! ```
//! # use std::{fs::File, path::Path, alloc::LayoutError};
//! # struct FileWithPath { file: File, path: Path }
//! impl FileWithPath {
//!     fn init_unsized<R>(file: File, path: &Path) -> R
//!     where
//!         R: dstify::SmartPointer<Self>
//!     {
//!         // ...
//!         # todo!()
//!     }
//!     fn init_unsized_checked<R>(file: File, path: &Path) -> Result<R, LayoutError>
//!     where
//!         R: dstify::SmartPointer<Self>
//!     {
//!         // ...
//!         # todo!()
//!     }
//! }
//! ```
//! ## Requirements
//! The type must be a `struct`. `enums` and `unions` are not supported as it's forbidden to define a dynamically-sized `enum` or `union` in current rust.
//! It must be annotated with `#[repr(C)]` and the last field *must* be a DST.
//! ```compile_fail
//! #[derive(Dstify)]
//! #[repr(C)]
//! struct Fail2Compile {
//!     not_dst: &[u64], // fails to compile due to last field not being a DST
//! }
//! ```
//! ```compile_fail
//! // fails to compile due to missing `#[repr(C)]`
//! #[derive(Dstify)]
//! struct Fail2Compile {
//!     dst: [u128],
//! }
//! ```
//!
//! ## Features
//!
//! - **"std"** - enabled by default  
//!   removing this feature (using `default-features = false`) enables `!#[no_std]` support.

#[doc(hidden)]
pub mod private;

mod smart_pointer;

pub use dstify_derive::Dstify;
pub use smart_pointer::SmartPointer;
