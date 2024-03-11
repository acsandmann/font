pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
  }
  
  impl Color {
    // #[allow(non_snake_case)]
    // pub const fn RGB(r: u8, g: u8, b: u8) -> Color { Color { r, g, b, a:
    // 0xff } }
    #[allow(non_snake_case)]
    pub const fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Color { Color { r, g, b, a } }
  
    #[inline]
    pub fn display(self) -> u32 { u32::from_be_bytes([self.r, self.g, self.b, self.a]) }
  }