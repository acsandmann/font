use fontdue::{Font, FontSettings};
use nanoserde::SerJson;

use crate::{ffi, structs::*};

#[no_mangle]
pub unsafe extern "C" fn font_new(ptr: ffi::mem::buf, size: usize, scale: usize) -> font {
  ffi::ptr::pack(
    Font::from_bytes(ffi::io::load(ptr, size), FontSettings {
      scale: scale as f32,
      ..fontdue::FontSettings::default()
    })
    .unwrap_unchecked(),
  )
}

#[no_mangle]
pub unsafe extern "C" fn font_metrics(ptr: font, char_raw: usize, scale: usize) -> *mut metrics {
  ffi::ptr::pack(crate::structs::metrics::from((*ptr).metrics(core::char::from_u32_unchecked(char_raw as u32), scale as f32)))
}

#[no_mangle]
pub unsafe extern "C" fn font_metrics_buffer(ptr: *mut metrics) -> ffi::mem::buf { ffi::io::store((&(*ptr)).serialize_json().into_bytes()) }

#[no_mangle]
pub unsafe extern "C" fn font_has(font: font, char_raw: u32) -> u16 { (*font).lookup_glyph_index(core::char::from_u32_unchecked(char_raw)) }

#[no_mangle]
pub unsafe extern "C" fn font_rasterize(font: font, char_raw: u32, scale: f32) -> rasterize {
  let (metrics, buffer) = (*font).rasterize(core::char::from_u32_unchecked(char_raw), scale);

  ffi::ptr::pack(raster_result {
    buffer,
    metrics: crate::structs::metrics::from(metrics),
  })
}

#[no_mangle]
pub unsafe extern "C" fn font_rasterize_buffer(ptr: rasterize) -> ffi::mem::buf { ffi::io::peek(&(*ptr).buffer) }

#[no_mangle]
pub unsafe extern "C" fn font_rasterize_metrics(ptr: rasterize) -> ffi::mem::buf { ffi::io::store((&(*ptr).metrics).serialize_json().into_bytes()) }
