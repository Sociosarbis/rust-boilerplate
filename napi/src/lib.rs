#![deny(clippy::all)]

use napi::{Env, JsNumber, JsObject, JsString, JsUndefined, NapiValue};

#[macro_use]
extern crate napi_derive;

mod assign;

fn result_to_option<T, E>(result: Result<T, E>) -> Option<T> {
  if let Ok(v) = result {
    Some(v)
  } else {
    None
  }
}

fn get<T>(env: Env, obj: &JsObject, paths: &Vec<&str>) -> Option<T>
where
  T: NapiValue,
{
  if paths.is_empty() {
    return None;
  }
  let n = paths.len();
  if n == 1 {
    if let Ok(cur) = obj.get_property::<JsString, T>(env.create_string(paths[0]).unwrap()) {
      return Some(cur);
    }
  } else if let Ok(mut cur) =
    obj.get_property::<JsString, JsObject>(env.create_string(paths[0]).unwrap())
  {
    for &p in paths.iter().skip(1).take(n - 2) {
      if let Ok(next) = cur.get_property::<JsString, JsObject>(env.create_string(p).unwrap()) {
        cur = next;
      } else {
        break;
      }
    }
    return result_to_option(
      cur.get_property::<JsString, T>(env.create_string(paths[n - 1]).unwrap()),
    );
  }
  None
}

fn set<T>(env: Env, obj: &mut JsObject, paths: &Vec<&str>, value: T) -> napi::Result<()>
where
  T: NapiValue,
{
  if paths.is_empty() {
    return Ok(());
  }
  let n = paths.len();
  if n == 1 {
    obj.set_property(env.create_string(paths[0]).unwrap(), value)?;
  } else if let Ok(mut cur) =
    obj.get_property::<JsString, JsObject>(env.create_string(paths[0]).unwrap())
  {
    for &p in paths.iter().skip(1).take(n - 2) {
      cur = cur
        .get_property::<JsString, JsObject>(env.create_string(p).unwrap())
        .unwrap();
    }
    cur.set_property(env.create_string(paths[n - 1]).unwrap(), value)?;
  }
  Ok(())
}

#[napi]
pub fn sum(env: Env, mut a: JsObject, b: i32) -> napi::Result<i32> {
  set(env, &mut a, &vec!["value"], env.create_int32(3).unwrap())?;
  Ok(i32::try_from(get::<JsNumber>(env, &a, &vec!["value"]).unwrap())? + b)
}

#[napi(ts_args_type = "a: number[], b: number[]")]
pub fn distance(env: Env, a: JsObject, b: JsObject) -> napi::Result<f64> {
  if let Ok(n1) = a.get_array_length() {
    if let Ok(n2) = b.get_array_length() {
      if n1 >= 2 && n2 >= 2 {
        let x1 = f64::try_from(get::<JsNumber>(env, &a, &vec!["0"]).unwrap())?;
        let y1 = f64::try_from(get::<JsNumber>(env, &a, &vec!["1"]).unwrap())?;
        let x2 = f64::try_from(b.get_element::<JsNumber>(0)?)?;
        let y2 = f64::try_from(b.get_element::<JsNumber>(1)?)?;
        return Ok(((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt());
      }
    }
  }
  Err(napi::Error::from_status(napi::Status::InvalidArg))
}

macro_rules! get_element {
    ($e: ident, $a: ident, $i: expr, $either_typ: ident, $t1: ty) => {
      if let Ok(v) = $a.get_element::<$t1>($i) {
        Ok(napi::$either_typ::B(v))
      } else {
        Ok(napi::Either::A($e.get_undefined()?))
      }
    };
    ($e: ident, $a: ident, $i: expr, $t1: ty) => {
      get_element!($e, $a, $i, Either, $t1)
  };
}

#[napi(ts_args_type = "a: unknown[]", ts_return_type = "unknown")]
pub fn last(env: Env, a: JsObject) -> napi::Result<napi::Either<JsUndefined, JsObject>> {
  if a.is_array()? {
    let n = a.get_array_length()?;
    if n > 0 {
      return get_element!(env, a, n - 1, JsObject);
    }
  }
  Ok(napi::Either::A(env.get_undefined()?))
}
