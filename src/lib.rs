#![allow(non_camel_case_types)]
#![allow(unstable_name_collisions)]
#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![warn(clippy::perf)]
#![feature(decl_macro)]
#![feature(const_mut_refs)]
#![warn(clippy::complexity)]
#![warn(clippy::correctness)]
#![allow(non_camel_case_types)]
#![feature(downcast_unchecked)]
#![allow(non_upper_case_globals)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_slice_from_raw_parts_mut)]
#![feature(thin_box)]
//#![no_std]

extern crate alloc;

pub mod framebuffer;

pub mod ffi;

mod structs;

pub use structs::*;

mod color;

mod font;
pub use font::*;

mod layout;
pub use layout::*;

macro_rules! define_generic_destructors {
    ($(($name:ident => $type:ty)),+) => { $(#[no_mangle] unsafe extern "C" fn $name(ptr: $type) { let _ = alloc::boxed::Box::from_raw(ptr); })+ };
}

define_generic_destructors! {
(layout_rasterize_free => rd_ptr),
(font_rasterize_free => crate::structs::rasterize),
(font_metrics_free => *mut crate::structs::metrics),
(font_free => crate::structs::font)
}
