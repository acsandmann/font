#![allow(non_camel_case_types)]
use alloc::vec::Vec;

use fontdue::{layout::Layout, Font};
use minivec::MiniVec;
use nanoserde::{DeJson, SerJson};
use smallbox::{space::S1, SmallBox as Box};

#[derive(SerJson)]
pub struct outline_bounds {
  xmin: f32,
  ymin: f32,
  width: f32,
  height: f32,
}
#[derive(SerJson)]
pub struct metrics {
  xmin: i32,
  ymin: i32,
  width: usize,
  height: usize,
  advance_width: f32,
  advance_height: f32,
  bounds: outline_bounds,
}

impl From<fontdue::Metrics> for metrics {
  fn from(metrics: fontdue::Metrics) -> Self {
    metrics {
      xmin: metrics.xmin,
      advance_height: metrics.advance_height,
      ymin: metrics.ymin,
      advance_width: metrics.advance_width,
      height: metrics.height,
      width: metrics.width,
      bounds: outline_bounds {
        xmin: metrics.bounds.xmin,
        ymin: metrics.bounds.ymin,
        width: metrics.bounds.width,
        height: metrics.bounds.height,
      },
    }
  }
}

#[derive(DeJson)]
pub enum horizontal_align {
  left,
  center,
  right,
}

#[derive(DeJson)]
pub enum vertical_align {
  top,
  middle,
  bottom,
}

#[derive(DeJson)]
pub enum wrap_style {
  word,
  letter,
}

#[derive(DeJson)]
pub struct layout_settings {
  // pub x: f32,
  // pub y: f32,
  pub max_width: Option<f32>,
  pub max_height: Option<f32>,
  pub horizontal_align: Option<horizontal_align>,
  pub vertical_align: Option<vertical_align>,
  pub wrap_style: Option<wrap_style>,
  pub wrap_hard_breaks: Option<bool>,
}

pub type font = *mut Font;
#[allow(type_alias_bounds)]
pub type layout<U: Copy + Clone = ()> = *mut Box<l_box<U>, S1>;
pub type rasterize = *mut raster_result;
pub type rd_ptr = *mut raster_data;

pub struct raster_result {
  pub buffer: Vec<u8>,
  pub metrics: metrics,
}

pub struct raster_data {
  pub buffer: Vec<u8>,
  pub width: u32,
  pub height: u32,
}

pub struct l_box<U: Copy + Clone = ()> {
  pub layout: Layout<U>,
  pub fonts: MiniVec<Font>,
}

impl l_box {
  pub fn new(f: Layout) -> l_box { l_box { layout: f, fonts: MiniVec::new() } }

  pub fn clear(&mut self) {
    self.layout.clear();
    self.fonts.clear();
  }

  #[inline]
  pub fn add_font(&mut self, f: Font) -> usize {
    self.fonts.push(f.clone());
    return self.fonts.len() - 1;
  }
}

impl l_box<u32> {
  #[inline]
  pub fn add_font(&mut self, f: Font) -> usize {
    self.fonts.push(f.clone());
    return self.fonts.len() - 1;
  }
}
