use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use jni::signature::{Primitive, ReturnType};
use jni::sys::jint;

pub fn hello() {
  println!("Hello World!")
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
  if let Ok(Ok(app)) = env.get_field(plugin, "app", "Lkt/lib/App;").map(|it| it.l()) {
    let _ = env.set_field(app, "sumResult", "I", JValue::Int(result));
  }
}
