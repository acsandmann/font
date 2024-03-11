#![allow(non_camel_case_types)]
#![allow(unstable_name_collisions)]
#![allow(clippy::missing_safety_doc)]
#![no_std]

extern crate alloc;

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
(layout_free => crate::structs::layout),
(font_rasterize_free => crate::structs::rasterize),
(font_metrics_free => *mut crate::structs::metrics),
(font_free => crate::structs::font)
}