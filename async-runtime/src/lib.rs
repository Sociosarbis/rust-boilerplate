extern crate libc;
use std::{
  cell::{Cell, UnsafeCell},
  future::Future,
  io::{ErrorKind, Read, Write},
  ops::Deref,
  os::fd::AsRawFd,
  task::Waker,
  time::Duration,
};

mod epoll;
mod event;
mod poll_event;
mod runner;
mod timer;
mod utils;

use epoll::*;
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
  let timer = Timer::new_timeout(dur).unwrap();
  let recv = (Cell::new(false), Cell::new(None));
  let runner = runner();
  let mut handle = std::pin::pin!(runner.epoll.listen_read(&timer, &recv).unwrap());
  while timer.ticks().unwrap() == 0 {
    handle.as_mut().await;
  }
}

#[test]
fn test_block_on() {
  block_on(async {
    println!("Hello");
    sleep(Duration::from_secs(1)).await;
    println!("World!");
  });
}
