extern crate libc;
use std::{
  alloc::Layout,
  cell::{Cell, UnsafeCell},
  future::Future,
  io::{ErrorKind, Read, Write},
  ops::Deref,
  os::fd::AsRawFd,
  ptr,
  task::Waker,
  time::Duration,
};

mod epoll;
mod event;
mod poll_event;
mod runner;
mod scope_guard;
mod timer;
mod utils;

use epoll::*;
use event::Event;
use runner::*;
use timer::*;

thread_local! {
  static LOCAL_RUNNER: UnsafeCell<Runner> = UnsafeCell::new(Runner::new().unwrap());
}

async fn read<R: ?Sized + AsRawFd + Read>(
  poll: &Epoll,
  reader: &mut R,
  buf: &mut [u8],
) -> std::io::Result<usize> {
  let flag: (Cell<bool>, Cell<Option<Waker>>) = (Cell::new(false), Cell::new(None));
  let mut handle = std::pin::pin!(poll.listen_read(reader, &flag)?);
  loop {
    match reader.read(buf) {
      Ok(x) => return Ok(x),
      Err(ref e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::Interrupted) => {
        handle.as_mut().await;
      }
      Err(e) => return Err(e),
    }
  }
}

async fn write<W: ?Sized + AsRawFd + Write>(
  poll: &Epoll,
  writer: &mut W,
  buf: &[u8],
) -> std::io::Result<usize> {
  let flag: (Cell<bool>, Cell<Option<Waker>>) = (Cell::new(false), Cell::new(None));
  let mut handle = std::pin::pin!(poll.listen_write(writer, &flag)?);
  loop {
    match writer.write(buf) {
      Ok(x) => return Ok(x),
      Err(ref e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::Interrupted) => {
        handle.as_mut().await;
      }
      Err(e) => return Err(e),
    }
  }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct RunnerRef(*const Runner);

impl Deref for RunnerRef {
  type Target = Runner;

  #[inline]
  fn deref(&self) -> &Self::Target {
    unsafe { &*self.0 }
  }
}

pub fn block_on<Fut: Future>(fut: Fut) -> Fut::Output {
  with_runner(|runner| runner.block_on(fut).unwrap())
}

pub(crate) fn runner() -> RunnerRef {
  with_runner(|r| RunnerRef(r))
}

pub(crate) fn with_runner<F: FnOnce(&Runner) -> T, T>(f: F) -> T {
  LOCAL_RUNNER.with(|runner| f(unsafe { &*runner.get() }))
}

pub async fn sleep(dur: Duration) {
  let (timer, handle) = Timer::new_timeout(dur).unwrap();
  let _guard = scope_guard::ScopeGuard::new(handle, |h| {
    let _ = h.join();
  });
  timer.await;
}

#[test]
fn test_block_on() {
  block_on(async {
    println!("Hello");
    sleep(Duration::from_secs(1)).await;
    println!("World!");
  });
}

#[test]
fn test_bump_alloc() {
  let bump_layout = Layout::from_size_align(3, 64).unwrap();
  let ptr = unsafe { std::alloc::alloc(bump_layout) };
  unsafe { ptr::write(ptr, 1) };
  let event_ptr = unsafe { ptr.add(1) as *mut event::Event };
  unsafe { ptr::write(event_ptr, Event::new().unwrap()) };
  let head_value = unsafe { ptr::read(ptr as *const _) };
  println!("head_value:{:?}", head_value);
  let event_value = unsafe { ptr::read(ptr.add(1) as *const Event) };
  println!("event_value:{:?}", event_value);

  unsafe {
    std::alloc::dealloc(ptr, bump_layout);
  }
}

const MAX_SIZE: usize = core::mem::size_of::<String>();
const LENGTH_MASK: u8 = 0b11000000;
const USIZE_SIZE: usize = core::mem::size_of::<usize>();
const VALID_MASK: usize = {
  let mut bytes = [255; USIZE_SIZE];
  bytes[USIZE_SIZE - 1] = 0;
  // 本机小端序
  usize::from_ne_bytes(bytes)
};
const HEAP_MARKER: usize = {
  let mut bytes = [0; USIZE_SIZE];
  bytes[USIZE_SIZE - 1] = 216;
  usize::from_ne_bytes(bytes)
};
const MAX_VALUE: usize = {
  let mut bytes = [255; USIZE_SIZE];
  bytes[USIZE_SIZE - 1] = 0;
  usize::from_le_bytes(bytes) - 1
};

#[test]
fn test_compact_str() {
  println!("max_size:{:?}", MAX_SIZE);
  let s = "abcdefghijklmnopqrstuvw";
  let mut buffer = [0u8; MAX_SIZE];
  // 即便字符串的长度等于24，由于utf-8字符的结束字节只会是0b10开头
  // 所以能够判断最后一位是否覆盖
  buffer[MAX_SIZE - 1] = s.len() as u8 | LENGTH_MASK;
  unsafe {
    ptr::copy_nonoverlapping(s.as_ptr(), buffer.as_mut_ptr(), s.len());
  }
  println!("buffer:{:?}", buffer);
  let s = unsafe {
    core::str::from_utf8_unchecked(core::slice::from_raw_parts(
      &buffer as *const _,
      buffer[MAX_SIZE - 1].wrapping_sub(LENGTH_MASK) as _,
    ))
  };
  println!("come back:{:?}", s);
}

const UNKNOWN: usize = 0;
type StrBuffer = [u8; UNKNOWN];

#[repr(C)]
struct HeapBufferInnerHeapCapacity {
  capacity: usize,
  buffer: StrBuffer,
}

pub fn layout(capacity: usize) -> std::alloc::Layout {
  let buffer_layout = std::alloc::Layout::array::<u8>(capacity).expect("valid capacity");
  std::alloc::Layout::new::<HeapBufferInnerHeapCapacity>()
    .extend(buffer_layout)
    .expect("valid layout")
    .0
    .pad_to_align()
}

#[test]
fn test_compact_str_on_heap() {
  println!("USIZE_SIZE:{:?}", USIZE_SIZE);
  println!("VALID_MASK:{:?}", VALID_MASK);
  println!("HEAP_MARKER:{:?}", HEAP_MARKER);
  println!("layout:{:?}", layout(MAX_VALUE + 1));
}
