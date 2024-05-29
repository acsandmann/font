#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct rgb {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct rgba {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl From<rgba> for rgb {
  #[inline]
  fn from(c: rgba) -> rgb { return rgb { r: c.r, g: c.g, b: c.b }; }
}
impl From<rgb> for rgba {
  #[inline]
  fn from(c: rgb) -> rgba { return rgba { r: c.r, g: c.g, b: c.b, a: 255 }; }
}
impl From<rgb> for u32 {
  #[inline]
  fn from(c: rgb) -> u32 { return c.r as u32 | ((c.g as u32) << 8) | ((c.b as u32) << 16) | (255 << 24); }
}
impl From<rgba> for u32 {
  #[inline]
  fn from(c: rgba) -> u32 { return c.r as u32 | ((c.g as u32) << 8) | ((c.b as u32) << 16) | ((c.a as u32) << 24); }
}
impl From<u32> for rgb {
  #[inline]
  fn from(c: u32) -> rgb {
    return rgb {
      r: (c & 0xFF) as u8,
      g: ((c >> 8) & 0xFF) as u8,
      b: ((c >> 16) & 0xFF) as u8,
    };
  }
}
impl From<u32> for rgba {
  #[inline]
  fn from(c: u32) -> rgba {
    return rgba {
      r: (c & 0xFF) as u8,
      g: ((c >> 8) & 0xFF) as u8,
      b: ((c >> 16) & 0xFF) as u8,
      a: (c >> 24) as u8,
    };
  }
}

pub fn blend(bg: u32, fg: u32) -> u32 {
  let fa = fg >> 24;
  let alpha = 1 + fa;
  let inv_alpha = 256 - fa;
  let r = (alpha * (fg & 0xff) + inv_alpha * (bg & 0xff)) >> 8;
  let g = (alpha * ((fg >> 8) & 0xff) + inv_alpha * ((bg >> 8) & 0xff)) >> 8;
  let b = (alpha * ((fg >> 16) & 0xff) + inv_alpha * ((bg >> 16) & 0xff)) >> 8;
  return r | ((g & 0xff) << 8) | ((b & 0xff) << 16) | (fa.max(bg >> 24) << 24);
}
