#![allow(non_camel_case_types)]
use alloc::vec::Vec;
use core::borrow::BorrowMut;

use fontdue::layout::{CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign, WrapStyle};
use image::{imageops::overlay_bounds, GenericImage, GenericImageView, GrayImage, ImageBuffer, Pixel, Rgba, RgbaImage};
use nanoserde::DeJson;
use smallbox::SmallBox as Box;

use crate::{color::Color, structs::*};
#[allow(non_upper_case_globals)]
const empty: Rgba<u8> = Rgba([0, 0, 0, 0]);

#[inline]
unsafe fn overlay<I, J>(bottom: &mut I, top: &J, x: u32, y: u32)
where
  I: GenericImage,
  J: GenericImageView<Pixel = I::Pixel>, {
  let c = overlay_bounds(bottom.dimensions(), top.dimensions(), x, y); // (range_width, range_height)
  (0..c.1).for_each(|top_y| {
    (0..c.0).for_each(|top_x| {
      let x = top_x + x;
      let y = top_y + y;
      let mut bottom_pixel = bottom.unsafe_get_pixel(x, y);
      bottom_pixel.blend(&top.unsafe_get_pixel(top_x, top_y));
      bottom.unsafe_put_pixel(x, y, bottom_pixel);
    });
  });
}

#[no_mangle]
unsafe extern "C" fn layout_new() -> layout { crate::ffi::ptr::pack(Box::new(l_box::new(Layout::new(CoordinateSystem::PositiveYDown)))) }

#[no_mangle]
unsafe extern "C" fn layout_clear(l: layout) { (*l).clear(); }

#[no_mangle]
unsafe extern "C" fn layout_lines(l: layout) -> usize {
  match (*l).layout.lines() {
    None => 0,
    Some(lp) => lp.len(),
  }
}

#[no_mangle]
unsafe extern "C" fn layout_reset(l: layout, ptr: crate::ffi::mem::buf, size: usize) {
  let layout: &mut Layout = (*l).layout.borrow_mut();
  let options: layout_settings = DeJson::deserialize_json(&crate::ffi::io::str(ptr, size)).unwrap_unchecked();
  layout.reset(&LayoutSettings {
    horizontal_align: match options.horizontal_align {
      Some(h) => match h {
        horizontal_align::center => HorizontalAlign::Center,
        horizontal_align::right => HorizontalAlign::Right,
        horizontal_align::left => HorizontalAlign::Left,
      },
      None => HorizontalAlign::Left,
    },
    vertical_align: match options.vertical_align {
      Some(v) => match v {
        vertical_align::middle => VerticalAlign::Middle,
        vertical_align::bottom => VerticalAlign::Bottom,
        vertical_align::top => VerticalAlign::Top,
      },
      None => VerticalAlign::Top,
    },
    max_width: options.max_width,
    max_height: options.max_height,
    wrap_hard_breaks: options.wrap_hard_breaks.unwrap_or(true),
    wrap_style: match options.wrap_style {
      Some(w) => match w {
        wrap_style::word => WrapStyle::Word,
        wrap_style::letter => WrapStyle::Letter,
      },
      None => WrapStyle::Word,
    },
    x: 0f32,
    y: 0f32,
    line_height: 1.0, // default?
  });
}

#[allow(clippy::many_single_char_names)]
#[no_mangle]
unsafe extern "C" fn layout_append(l: layout, fptr: font, ptr: crate::ffi::mem::buf, size: usize, scale: f32, has_color: bool, r: u8, g: u8, b: u8, a: u8) {
  let l: layout<u32> = core::mem::transmute(l);
  let len = (*l).add_font((&(*fptr)).clone());
  let c = if has_color { Color::RGBA(r, g, b, a).display() } else { 0 };
  (*l).layout.append(&(*l).fonts, &(TextStyle::with_user_data(crate::ffi::io::str(ptr, size), scale, len, c)));
}

#[no_mangle]
unsafe extern "C" fn layout_rasterize(l: layout<u32>, r: u8, g: u8, b: u8, a: u8) -> rd_ptr {
  // let height = (*l).layout.height() as u32;
  let fonts = &(*l).fonts;
  let glyphs = &(*l).layout.glyphs();
  let mm1 = glyphs.iter().map(|x| x.width as u32).collect::<Vec<u32>>();
  let mm2 = glyphs.iter().map(|x| x.x as u32).collect::<Vec<u32>>();
  let mm3 = glyphs.iter().map(|x| x.height as u32).collect::<Vec<u32>>();
  let mm4 = glyphs.iter().map(|x| x.y as u32).collect::<Vec<u32>>();
  let width = mm1.iter().max().unwrap_unchecked() + mm2.iter().max().unwrap_unchecked();
  let height = mm3.iter().max().unwrap_unchecked() + mm4.iter().max().unwrap_unchecked();
  // let height = (l_g.height + l_g.y as usize) as u32;
  let mut opacity = GrayImage::new(width, height);
  glyphs.iter().for_each(|&glyph| {
    let (metrics, bitmap) = fonts.get_unchecked(glyph.font_index).rasterize_config(glyph.key);
    let bitmap = ImageBuffer::from_vec(metrics.width as u32, metrics.height as u32, bitmap).unwrap_unchecked();
    overlay(&mut opacity, &bitmap, glyph.x as u32, glyph.y as u32)
  });
  // Rgb::from([r, g, b]).to_rgba();
  let mut cell = RgbaImage::from_pixel(width, height, empty);
  cell.pixels_mut().zip(opacity.pixels().map(|&pixel| pixel.0[0])).for_each(|(pixel, opacity)| {
    if opacity != 0 {
      let s = &mut pixel.channels_mut()[..];
      core::ptr::copy((&[r, g, b, opacity & a]).as_ptr(), s.as_mut_ptr(), s.len());
    }
  });

  crate::ffi::ptr::pack(raster_data {
    buffer: cell.into_raw(),
    width,
    height,
  })
}

#[no_mangle]
unsafe extern "C" fn layout_rasterize_width(l: rd_ptr) -> u32 { (*l).width }
#[no_mangle]
unsafe extern "C" fn layout_rasterize_height(l: rd_ptr) -> u32 { (*l).height }
#[no_mangle]
unsafe extern "C" fn layout_rasterize_buffer(l: rd_ptr) -> crate::ffi::mem::buf { crate::ffi::io::peek(&(*l).buffer) }
