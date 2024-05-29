use crate::ffi::mem;
static mut token: u64 = 0;

#[no_mangle]
pub unsafe extern "C" fn wlen() -> usize { mem::length() }
#[no_mangle]
pub unsafe extern "C" fn wtoken() -> u64 {
  token += 1;
  token - 1
}
#[no_mangle]
pub unsafe extern "C" fn wfree(ptr: mem::buf, size: usize) { mem::free(ptr, size); }
#[no_mangle]
pub unsafe extern "C" fn walloc(size: usize) -> mem::buf { mem::alloc(size) }
