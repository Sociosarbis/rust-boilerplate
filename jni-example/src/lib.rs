use std::alloc::{Layout, dealloc};
use std::ops::{Deref, DerefMut};
use std::ptr;

use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use jni::signature::{Primitive, ReturnType};
use jni::sys::jint;

struct Pointer<T>
where
  *mut T: CallDrop,
{
  ptr: *mut T,
}

impl<T> Pointer<T>
where
  *mut T: CallDrop,
{
  fn new(ptr: *mut T) -> Self {
    Pointer { ptr }
  }

  fn ptr(&self) -> *const T {
    self.ptr
  }

  fn mut_ptr(&self) -> *mut T {
    self.ptr
  }
}

impl<T> Deref for Pointer<T>
where
  *mut T: CallDrop,
{
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.ptr }
  }
}

impl<T> DerefMut for Pointer<T>
where
  *mut T: CallDrop,
{
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.ptr }
  }
}

trait CallDrop {
  fn call_drop(self);
}

impl<T> Drop for Pointer<T>
where
  *mut T: CallDrop,
{
  fn drop(&mut self) {
    <*mut T>::call_drop(self.ptr);
  }
}

impl CallDrop for *mut &str {
  fn call_drop(self) {
    unsafe {
      ptr::drop_in_place(self);
      dealloc(self.cast::<u8>(), Layout::new::<&str>());
      println!("dropped");
    }
  }
}

pub fn hello() {
  let str = Box::new("Hello World!");
  let s = Pointer::new(Box::into_raw(str));
  println!("{}", *s);
  println!("end");
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_kt_lib_Plugin_sum<'local>(
  mut env: JNIEnv<'local>,
  _plugin: JObject<'local>,
  nums: JObject<'local>,
) -> jint {
  if let Ok(list) = env.get_list(&nums) {
    if let Ok(size) = list.size(&mut env) {
      if let Ok(integer_class) = env.find_class("java/lang/Integer") {
        if let Ok(int_value_method_id) = env.get_method_id(integer_class, "intValue", "()I") {
          return (0..size)
            .map(|i| {
              if let Ok(Some(it)) = list.get(&mut env, i) {
                unsafe {
                  if let Ok(Ok(res)) = env
                    .call_method_unchecked(
                      it,
                      int_value_method_id,
                      ReturnType::Primitive(Primitive::Int),
                      &[],
                    )
                    .map(|it| it.i())
                  {
                    return res;
                  }
                }
              }
              0
            })
            .sum();
        }
      }
    }
  }
  0
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_kt_lib_Plugin_setResult<'local>(
  mut env: JNIEnv<'local>,
  plugin: JObject<'local>,
  result: jint,
) {
  if let Ok(Ok(app)) = env
    .get_field(plugin, "app", "Lkt/lib/App;")
    .map(|it| it.l())
  {
    let _ = env.set_field(app, "sumResult", "I", JValue::Int(result));
  }
}
