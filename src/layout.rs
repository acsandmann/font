#![allow(non_camel_case_types)]
use core::borrow::BorrowMut;

use fontdue::layout::{CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign, WrapStyle};
use nanoserde::DeJson;
use smallbox::SmallBox as Box;

use crate::{color::Color, framebuffer::framebuffer as fb, structs::*};
#[allow(non_upper_case_globals)]
const EMPTY: [u8; 4] = [0, 0, 0, 0];

#[no_mangle]
unsafe extern "C" fn layout_new() -> layout { crate::ffi::ptr::pack(Box::new(l_box::new(Layout::new(CoordinateSystem::PositiveYDown)))) }

#[no_mangle]
unsafe extern "C" fn layout_free(l: layout) { let _ = alloc::boxed::Box::from_raw(l.into()); }

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
    let fonts = &(*l).fonts;
    let glyphs = &(*l).layout.glyphs();

    let mut max_width = 0;
    let mut max_x = 0;
    let mut max_height = 0;
    let mut max_y = 0;
    let mut max_scale = glyphs[0].key.px;

    for glyph in glyphs.iter() {
        let width = glyph.width as usize;
        let x = glyph.x as usize;
        let height = glyph.height as usize;
        let y = glyph.y as usize;
        let scale = glyph.key.px;

        if width > max_width {
            max_width = width;
        }
        if x > max_x {
            max_x = x;
        }
        if height > max_height {
            max_height = height;
        }
        if y > max_y {
            max_y = y;
        }
        if scale > max_scale {
            max_scale = scale;
        }
    }

    let width = max_width + max_x;
    let height = max_height + max_y;
    let scale = max_scale;
    let bottom_padding = (scale * 0.2) as usize;
    let padded_height = height + bottom_padding;
    let mut framebuffer = fb::new(width, padded_height);

    glyphs.iter().for_each(|&glyph| {
        let (metrics, bitmap) = fonts.get_unchecked(glyph.font_index).rasterize_config(glyph.key);

        for y in 0..metrics.height as usize {
            for x in 0..metrics.width as usize {
                let bitmap_index = y * metrics.width as usize + x;
                let alpha = *bitmap.get_unchecked(bitmap_index);

                let framebuffer_x = glyph.x as usize + x;
                let framebuffer_y = glyph.y as usize + y;
                if framebuffer_x < width && framebuffer_y < padded_height - bottom_padding {
                    let framebuffer_index = (framebuffer_y * width + framebuffer_x) * 4;
                    let pixel = core::slice::from_raw_parts_mut(framebuffer.as_mut_ptr::<u8>().add(framebuffer_index), 4);
                    pixel[0] = (r as u16 * alpha as u16 / 255) as u8;
                    pixel[1] = (g as u16 * alpha as u16 / 255) as u8;
                    pixel[2] = (b as u16 * alpha as u16 / 255) as u8;
                    pixel[3] = (a as u16 * alpha as u16 / 255) as u8;
                }
            }
        }
    });

    let (w, h) = (framebuffer.width, framebuffer.height);
    crate::ffi::ptr::pack(raster_data {
        buffer: framebuffer.into_vec(),
        width: w as u32,
        height: h as u32,
    })
}

#[no_mangle]
unsafe extern "C" fn layout_rasterize_width(l: rd_ptr) -> u32 { (*l).width }
#[no_mangle]
unsafe extern "C" fn layout_rasterize_height(l: rd_ptr) -> u32 { (*l).height }
#[no_mangle]
unsafe extern "C" fn layout_rasterize_buffer(l: rd_ptr) -> crate::ffi::mem::buf { crate::ffi::io::peek(&(*l).buffer) }
