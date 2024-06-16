// copied from https://github.com/ArroyoSystems/arroyo
use std::time::Duration;

use arrow::datatypes::UInt32Type;
use common::{ArrowDatum, DrainResult, FfiArrays, SendableFfiAsyncUdfHandle};
use plugin::{drain_results, send, stop_runtime, AsyncUdf};
use tokio;

mod common;
mod plugin;

async fn my_udf(a: u64, b: String) -> u32 {
  tokio::time::sleep(Duration::from_millis(10)).await;
  a as u32 + b.len() as u32
}

async fn __wrapper(
  id: u64,
  timeout: std::time::Duration,
  args: Vec<arrow::array::ArrayData>,
) -> (u64, Result<ArrowDatum, tokio::time::error::Elapsed>) {
  let mut args = args.into_iter();
  let arg_0 =
    arrow::array::PrimitiveArray::<arrow::datatypes::UInt64Type>::from(args.next().unwrap());
  let arg_1 = arrow::array::StringArray::from(args.next().unwrap());

  match tokio::time::timeout(
    Duration::from_secs(5),
    my_udf(arg_0.value(0), arg_1.value(0).to_string()),
  )
  .await
  {
    Ok(result) => (id, Ok(ArrowDatum::U32(Some(result)))),
    Err(e) => (id, Err(e)),
  }
}

#[no_mangle]
pub extern "C-unwind" fn __start(
  ordered: bool,
  timeout_micros: u64,
  allowed_in_flight: u32,
) -> SendableFfiAsyncUdfHandle {
  let (x, handle) = AsyncUdf::new(
    ordered,
    Duration::from_micros(timeout_micros),
    allowed_in_flight,
    Box::new(arrow::array::PrimitiveBuilder::<UInt32Type>::new()),
    __wrapper,
  );
  x.start();
  SendableFfiAsyncUdfHandle {
    ptr: handle.into_ffi(),
  }
}

#[no_mangle]
pub extern "C-unwind" fn __send(
  handle: SendableFfiAsyncUdfHandle,
  id: u64,
  arrays: FfiArrays,
) -> async_ffi::FfiFuture<bool> {
  use async_ffi::FutureExt;
  send(handle, id, arrays).into_ffi()
}

#[no_mangle]
pub extern "C-unwind" fn __drain_results(handle: SendableFfiAsyncUdfHandle) -> DrainResult {
  drain_results(handle)
}

#[no_mangle]
pub extern "C-unwind" fn __stop_runtime(handle: SendableFfiAsyncUdfHandle) {
  stop_runtime(handle);
}

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
