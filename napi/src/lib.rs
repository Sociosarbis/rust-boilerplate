#![deny(clippy::all)]

use napi::{Env, JsNumber, JsObject, JsString, NapiValue};

#[macro_use]
extern crate napi_derive;

fn get<T>(env: Env, mut obj: JsObject, paths: &Vec<&str>) -> Option<T>
where
  T: NapiValue,
{
  if paths.is_empty() {
    return None;
  }
  let n = paths.len();
  for &p in paths.iter().take(n - 1) {
    obj = obj
      .get_property::<JsString, JsObject>(env.create_string(p).unwrap())
      .unwrap();
  }
  Some(
    obj
      .get_property::<JsString, T>(env.create_string(paths[n - 1]).unwrap())
      .unwrap(),
  )
}

fn set<T>(env: Env, mut obj: JsObject, paths: &Vec<&str>, value: T) -> JsObject
where
  T: NapiValue,
{
  if paths.is_empty() {
    return obj;
  }
  let n = paths.len();
  for &p in paths.iter().take(n - 1) {
    obj = obj
      .get_property::<JsString, JsObject>(env.create_string(p).unwrap())
      .unwrap();
  }
  let _ = obj.set_property(env.create_string(paths[n - 1]).unwrap(), value);
  obj
}

#[napi]
pub fn sum(env: Env, mut a: JsObject, b: i32) -> i32 {
  a = set(env, a, &vec!["value"], env.create_int32(3).unwrap());
  i32::try_from(get::<JsNumber>(env, a, &vec!["value"]).unwrap()).unwrap() + b
}
