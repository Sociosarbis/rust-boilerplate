use std::{
  cell::{Cell, UnsafeCell},
  io::ErrorKind,
  os::fd::{AsRawFd, FromRawFd, OwnedFd},
  task::Waker,
};

use super::poll_event::*;
use crate::utils::*;

pub struct Epoll {
  fd: OwnedFd,
  size: Cell<usize>,
  sink: UnsafeCell<Vec<libc::epoll_event>>,
}

#[repr(C)]
pub union epoll_data {
  pub ptr: *mut libc::c_void,
  pub fd: libc::c_int,
  pub u32: u32,
  pub u64: u64,
}

impl Epoll {
  pub fn listen_read<'a, 'b, T: ?Sized + AsRawFd>(
    &'a self,
    fd: &T,
    recv: &'b (Cell<bool>, Cell<Option<Waker>>),
  ) -> std::io::Result<PollEvent<'a, 'b>> {
    self.listen(fd, recv, libc::EPOLLIN as _)
  }

  pub fn listen_write<'a, 'b, T: ?Sized + AsRawFd>(
    &'a self,
    fd: &T,
    recv: &'b (Cell<bool>, Cell<Option<Waker>>),
  ) -> std::io::Result<PollEvent<'a, 'b>> {
    self.listen(fd, recv, libc::EPOLLOUT as _)
  }

  fn listen<'a, 'b, T: ?Sized + AsRawFd>(
    &'a self,
    fd: &T,
    recv: &'b (Cell<bool>, Cell<Option<Waker>>),
    events: u32,
  ) -> std::io::Result<PollEvent<'a, 'b>> {
    let fd = fd.as_raw_fd();
    let mut evt = libc::epoll_event {
      events: events | libc::EPOLLET as u32,
      u64: unsafe {
        epoll_data {
          ptr: recv as *const _ as _,
        }
        .u64
      },
    };
    match unsafe {
      err_handle(libc::epoll_ctl(
        self.fd.as_raw_fd(),
        libc::EPOLL_CTL_ADD,
        fd,
        &mut evt,
      ))
    } {
      Ok(_) => {
        self.size.set(self.size.get() + 1);
        Ok(PollEvent::new(fd, recv, self))
      }
      Err(e) => Err(e),
    }
  }

  pub fn unlisten(&self, fd: i32) -> std::io::Result<libc::c_int> {
    self.size.set(self.size.get() - 1);
    unsafe {
      err_handle(libc::epoll_ctl(
        self.as_raw_fd(),
        libc::EPOLL_CTL_DEL,
        fd,
        core::ptr::null_mut(),
      ))
    }
  }
}

impl Epoll {
  pub fn new() -> std::io::Result<Self> {
    let fd = unsafe { err_handle(libc::epoll_create1(libc::EPOLL_CLOEXEC))? };
    Ok(Self {
      fd: unsafe { OwnedFd::from_raw_fd(fd) },
      size: Cell::new(0),
      sink: UnsafeCell::new(Vec::new()),
    })
  }

  /** 接收事件后就会解除block */
  pub fn wait(&self) -> std::io::Result<()> {
    let size = self.size.get();
    let sink = unsafe { &mut *self.sink.get() };
    sink.reserve(1);
    println!("fd:{:?}", self.fd.as_raw_fd());
    let res = unsafe {
      loop {
        match err_handle(libc::epoll_wait(
          self.fd.as_raw_fd(),
          sink.as_mut_ptr(),
          size as _,
          -1,
        )) {
          Ok(x) => break x,
          Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
          Err(e) => {
            println!("wait error");
            return Err(e);
          }
        }
      }
    };

    unsafe { sink.set_len(res as _) };

    for evt in sink.drain(..) {
      let data: &(Cell<bool>, Cell<Option<Waker>>) =
        unsafe { &*epoll_data { u64: evt.u64 }.ptr.cast() };

      data.0.set(true);
      if let Some(waker) = data.1.take() {
        waker.wake()
      }
    }

    std::io::Result::Ok(())
  }
}

impl AsRawFd for Epoll {
  fn as_raw_fd(&self) -> std::os::fd::RawFd {
    self.fd.as_raw_fd()
  }
}
