use std::{
  io::ErrorKind,
  os::fd::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd}, task::{RawWakerVTable, RawWaker, Waker}, mem::ManuallyDrop,
};

use crate::utils::err_handle;

#[derive(Debug)]
pub struct Event(OwnedFd);

impl Event {
  pub fn new() -> std::io::Result<Self> {
    let fd = unsafe { err_handle(libc::eventfd(0, libc::EFD_NONBLOCK | libc::EFD_CLOEXEC))? };
    unsafe { Ok(Self(OwnedFd::from_raw_fd(fd))) }
  }

  pub fn notify(&self) -> std::io::Result<()> {
    let value = 1u64;
    unsafe {
      err_handle(
        libc::write(self.0.as_raw_fd(), std::ptr::addr_of!(value).cast(), 8) as _,
      )?;
    }
    Ok(())
  }

  pub fn try_clone(&self) -> std::io::Result<Self> {
    self.0.try_clone().map(Self)
  }

  pub fn try_get(&self) -> std::io::Result<bool> {
    let mut buf = 0u64;
    unsafe {
      match err_handle(
        libc::read(self.0.as_raw_fd(), std::ptr::addr_of_mut!(buf).cast(), 8) as _,
      ) {
        Ok(_) => Ok(true),
        Err(ref e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::Interrupted) => {
          Ok(false)
        }

        Err(e) => Err(e),
      }
    }
  }
}

impl IntoRawFd for Event {
  fn into_raw_fd(self) -> std::os::fd::RawFd {
    self.0.into_raw_fd()
  }
}

impl FromRawFd for Event {
  unsafe fn from_raw_fd(fd: std::os::fd::RawFd) -> Self {
    Self(OwnedFd::from_raw_fd(fd))
  }
}

impl AsRawFd for Event {
  fn as_raw_fd(&self) -> std::os::fd::RawFd {
    self.0.as_raw_fd()
  }
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

unsafe fn clone(data: *const ()) -> RawWaker {
  let signal = /* 避免源fd drop */ ManuallyDrop::new(Event::from_raw_fd(data as _));
  let new_signal = signal.try_clone().unwrap();
  raw_waker(new_signal)
}

unsafe fn wake(data: *const ()) {
  let signal = ManuallyDrop::new(Event::from_raw_fd(data as _));
  signal.notify().unwrap();
}

unsafe fn wake_by_ref(data: *const ()) {
  let signal = ManuallyDrop::new(Event::from_raw_fd(data as _));
  signal.notify().unwrap();
}

unsafe fn drop(data: *const ()) {
  std::mem::drop(Event::from_raw_fd(data as _));
}

fn raw_waker(signal: Event) -> RawWaker {
  let signal = signal.into_raw_fd();
  RawWaker::new(signal as *const _, &VTABLE)
}
// 在async-std中waker传入的是整个RawTask的指针，而RawTask包含future的指针
// 这样在wake的时候，通过ptr转换回RawTask，然后就可以访问到future，并重新poll
// 可以通过Pin::new_unchecked将future指针转回Pin<Future>
pub fn waker(signal: Event) -> Waker {
  unsafe { Waker::from_raw(raw_waker(signal)) }
}
