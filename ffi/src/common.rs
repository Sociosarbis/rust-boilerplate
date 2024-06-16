use std::{
  sync::{Arc, Mutex},
  time::{SystemTime, UNIX_EPOCH},
};

use arrow::{
  array::{
    ArrayBuilder, ArrayData, BinaryBuilder, BooleanBuilder, Float32Builder, Float64Builder,
    Int32Builder, Int64Builder, StringBuilder, TimestampNanosecondBuilder, UInt32Builder,
    UInt64Builder,
  },
  ffi::{from_ffi, to_ffi, FFI_ArrowSchema},
};

use arrow::ffi::FFI_ArrowArray;

pub type QueueData = (u64, Vec<ArrayData>);
pub type ResultMutex = Arc<Mutex<(UInt64Builder, Box<dyn ArrayBuilder>)>>;

pub enum ArrowDatum {
  Bool(Option<bool>),
  U32(Option<u32>),
  U64(Option<u64>),
  I32(Option<i32>),
  I64(Option<i64>),
  F32(Option<f32>),
  F64(Option<f64>),
  String(Option<String>),
  Bytes(Option<Vec<u8>>),
  Timestamp(Option<SystemTime>),
}

fn to_nanos(time: SystemTime) -> i64 {
  time.duration_since(UNIX_EPOCH).unwrap().as_nanos() as i64
}

impl ArrowDatum {
  pub fn append_to(self, builder: &mut dyn ArrayBuilder) {
    match self {
      ArrowDatum::Bool(x) => builder
        .as_any_mut()
        .downcast_mut::<BooleanBuilder>()
        .unwrap()
        .append_option(x),
      ArrowDatum::U32(x) => builder
        .as_any_mut()
        .downcast_mut::<UInt32Builder>()
        .unwrap()
        .append_option(x),
      ArrowDatum::U64(x) => builder
        .as_any_mut()
        .downcast_mut::<UInt64Builder>()
        .unwrap()
        .append_option(x),
      ArrowDatum::I32(x) => builder
        .as_any_mut()
        .downcast_mut::<Int32Builder>()
        .unwrap()
        .append_option(x),
      ArrowDatum::I64(x) => builder
        .as_any_mut()
        .downcast_mut::<Int64Builder>()
        .unwrap()
        .append_option(x),
      ArrowDatum::F32(x) => builder
        .as_any_mut()
        .downcast_mut::<Float32Builder>()
        .unwrap()
        .append_option(x),
      ArrowDatum::F64(x) => builder
        .as_any_mut()
        .downcast_mut::<Float64Builder>()
        .unwrap()
        .append_option(x),
      ArrowDatum::String(x) => builder
        .as_any_mut()
        .downcast_mut::<StringBuilder>()
        .unwrap()
        .append_option(x),
      ArrowDatum::Bytes(x) => builder
        .as_any_mut()
        .downcast_mut::<BinaryBuilder>()
        .unwrap()
        .append_option(x),
      ArrowDatum::Timestamp(x) => builder
        .as_any_mut()
        .downcast_mut::<TimestampNanosecondBuilder>()
        .unwrap()
        .append_option(x.map(to_nanos)),
    }
  }
}

// ArrayData在ffi间传递需要转换成这个类型保证ffi safe
#[repr(C)]
#[derive(Debug)]
pub struct FfiArraySchema(pub FFI_ArrowArray, pub FFI_ArrowSchema);

#[repr(C)]
pub struct FfiArrays {
  ptr: *mut FfiArraySchema,
  len: usize,
  capacity: usize,
  error: bool,
}

unsafe impl Send for FfiArrays {}

impl FfiArrays {
  pub fn from_vec(value: Vec<ArrayData>) -> Self {
    let vec: Vec<_> = value
      .into_iter()
      .map(|a| to_ffi(&a).unwrap())
      .map(|(data, schema)| FfiArraySchema(data, schema))
      .collect();
    let len = vec.len();
    let capacity = vec.capacity();
    let ptr = vec.leak().as_mut_ptr();
    Self {
      ptr,
      len,
      capacity,
      error: false,
    }
  }

  pub fn into_vec(self) -> Vec<ArrayData> {
    let vec = unsafe { Vec::from_raw_parts(self.ptr, self.len, self.capacity) };

    vec
      .into_iter()
      .map(|FfiArraySchema(array, schema)| unsafe { from_ffi(array, &schema).unwrap() })
      .collect()
  }
}

// [u8; 0]是用来表示opaque type，通常用来调用其实际类型的C函数
#[repr(C)]
pub struct FfiAsyncUdfHandle {
  _data: [u8; 0],
  _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SendableFfiAsyncUdfHandle {
  pub ptr: *mut FfiAsyncUdfHandle,
}

// 因为__send需要Send bound
unsafe impl Send for SendableFfiAsyncUdfHandle {}
unsafe impl Sync for SendableFfiAsyncUdfHandle {}

#[repr(C)]
pub enum DrainResult {
  Data(FfiArrays),
  None,
  Error,
}
