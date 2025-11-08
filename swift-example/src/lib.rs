use core::slice;
use std::{
  ffi::c_int,
  os::raw::c_void,
  ptr::{self},
  sync::OnceLock,
};

static PLUGIN_IMPL: OnceLock<PluginImpl> = OnceLock::new();

type Create = extern "C" fn() -> *mut c_void;
type SetResult = extern "C" fn(ptr: *mut c_void, result: c_int);

#[repr(C)]
struct PluginImpl {
  create: Create,
  set_result: SetResult,
}

#[unsafe(no_mangle)]
pub extern "C" fn register_plugin_impl(create: Create, set_result: SetResult) {
  PLUGIN_IMPL.get_or_init(|| PluginImpl { create, set_result });
}

#[unsafe(no_mangle)]
pub extern "C" fn create() -> *mut c_void {
  if let Some(methods) = PLUGIN_IMPL.get() {
    (methods.create)()
  } else {
    ptr::null_mut()
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn sum(ptr: *mut c_int, length: c_int) -> c_int {
  let nums = unsafe { slice::from_raw_parts(ptr, length as usize) };
  nums.iter().sum()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_result(ptr: *mut c_void, result: c_int) {
  if let Some(methods) = PLUGIN_IMPL.get() {
    (methods.set_result)(ptr, result);
  }
}
