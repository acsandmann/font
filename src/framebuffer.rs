#![allow(non_camel_case_types)]
macro size_of($type:ty) {
  core::mem::size_of::<$type>()
}
#[inline]
const fn alloc_len(size: usize) -> usize { return alloc_align(size).size(); }
#[inline(always)]
const unsafe fn unreachable() -> ! { core::hint::unreachable_unchecked() }
#[inline]
fn alloc(size: usize) -> *mut u8 {
  unsafe {
    return alloc::alloc::alloc(alloc_align(size));
  }
}
#[inline]
fn free(ptr: *mut u8, size: usize) {
  unsafe {
    alloc::alloc::dealloc(ptr, alloc_align(size));
  }
}
#[inline]
fn calloc(size: usize) -> *mut u8 {
  unsafe {
    return alloc::alloc::alloc_zeroed(alloc_align(size));
  }
}
#[inline]
const fn alloc_align(size: usize) -> core::alloc::Layout { return unsafe { core::alloc::Layout::from_size_align_unchecked(size, 16) }; }

type fb = framebuffer;

pub struct framebuffer {
  pub width: usize,
  pub height: usize,
  ptr: (bool, *mut u8),
}

unsafe impl Send for framebuffer {}
unsafe impl Sync for framebuffer {}

pub trait framebuffer_from<T> {
  fn from(width: usize, height: usize, container: T) -> fb;
}

impl Drop for framebuffer {
  fn drop(&mut self) {
    if self.ptr.0 {
      free(self.as_mut_ptr(), self.len());
    }
  }
}

impl Clone for framebuffer {
  fn clone(&self) -> Self {
    let ptr = alloc(self.len());
    unsafe {
      self.ptr.1.copy_to_nonoverlapping(ptr, self.len());
    }
    return Self {
      width: self.width,
      height: self.height,
      ptr: (true, ptr),
    };
  }
}

impl<T> framebuffer_from<*mut T> for *mut T {
  fn from(width: usize, height: usize, container: *mut T) -> fb {
    return fb {
      width,
      height,
      ptr: (false, container as *mut u8),
    };
  }
}
impl<T> framebuffer_from<&[T]> for &[T] {
  fn from(width: usize, height: usize, container: &[T]) -> fb {
    return fb {
      width,
      height,
      ptr: (false, container.as_ptr() as *mut u8),
    };
  }
}
impl<T> framebuffer_from<*const T> for *const T {
  fn from(width: usize, height: usize, container: *const T) -> fb {
    return fb {
      width,
      height,
      ptr: (false, container as *mut u8),
    };
  }
}
impl<T> framebuffer_from<&Vec<T>> for &Vec<T> {
  fn from(width: usize, height: usize, container: &Vec<T>) -> fb {
    return fb {
      width,
      height,
      ptr: (false, container.as_ptr() as *mut u8),
    };
  }
}
impl<T> framebuffer_from<&mut [T]> for &mut [T] {
  fn from(width: usize, height: usize, container: &mut [T]) -> fb {
    return fb {
      width,
      height,
      ptr: (false, container.as_mut_ptr() as *mut u8),
    };
  }
}
impl<T> framebuffer_from<&mut Vec<T>> for &mut Vec<T> {
  fn from(width: usize, height: usize, container: &mut Vec<T>) -> fb {
    return fb {
      width,
      height,
      ptr: (false, container.as_mut_ptr() as *mut u8),
    };
  }
}

impl framebuffer {
  pub fn new(width: usize, height: usize) -> Self {
    return Self {
      width,
      height,
      ptr: (true, calloc(4 * width * height)),
    };
  }

  pub fn from<T: framebuffer_from<T>>(width: usize, height: usize, container: T) -> Self { return T::from(width, height, container); }

  pub unsafe fn new_uninit(width: usize, height: usize) -> Self {
    return Self {
      width,
      height,
      ptr: (true, alloc(4 * width * height)),
    };
  }

  pub const fn len(&self) -> usize { return 4 * self.width * self.height; }

  pub const fn as_ptr<T>(&self) -> *const T { return self.ptr.1 as *const T; }

  pub const fn as_mut_ptr<T>(&mut self) -> *mut T { return self.ptr.1 as *mut T; }

  pub const fn as_slice<T>(&self) -> &[T] { return unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len() / size_of!(T)) }; }

  pub const fn as_mut_slice<T>(&mut self) -> &mut [T] { return unsafe { core::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len() / size_of!(T)) }; }

  pub fn into_vec<T>(mut self) -> Vec<T> {
    self.ptr.0 = false;
    return unsafe { Vec::from_raw_parts(self.as_mut_ptr(), self.len() / size_of!(T), alloc_len(self.len()) / size_of!(T)) };
  }
}
