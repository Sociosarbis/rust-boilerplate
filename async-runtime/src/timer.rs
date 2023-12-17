extern crate libc;
use libc::*;
use std::{
  future::Future,
  io::ErrorKind,
  os::fd::{AsRawFd, FromRawFd, OwnedFd},
  ptr::addr_of_mut,
  sync::{atomic::AtomicBool, Arc, Mutex},
  task::Waker,
  time::Duration, thread::JoinHandle,
};

use crate::utils::err_handle;

fn dur2timespec_saturating(delta: Duration) -> libc::timespec {
  libc::timespec {
    tv_sec: delta.as_secs() as _,
    tv_nsec: delta.subsec_nanos() as _,
  }
}

#[derive(Debug)]
struct State {
  is_finished: AtomicBool,
  waker: Option<Waker>,
}

#[derive(Debug)]
pub struct Timer(OwnedFd, Arc<Mutex<State>>);

impl Timer {
  fn new_(clockid: libc::c_int) -> std::io::Result<Self> {
    let fd = unsafe {
      err_handle(timerfd_create(
        clockid,
        libc::TFD_NONBLOCK | libc::TFD_CLOEXEC,
      ))?
    };
    unsafe {
      Ok(Self(
        OwnedFd::from_raw_fd(fd),
        Arc::new(Mutex::new(State {
          is_finished: AtomicBool::new(false),
          waker: None,
        })),
      ))
    }
  }

  fn setup_timeout(&mut self, delta: Duration) -> std::io::Result<()> {
    let value = libc::itimerspec {
      it_interval: libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      it_value: dur2timespec_saturating(delta),
    };
    unsafe {
      err_handle(libc::timerfd_settime(
        self.0.as_raw_fd(),
        0,
        &value,
        core::ptr::null_mut(),
      ))?;
    }
    Ok(())
  }

  pub fn new_timeout(until: Duration) -> std::io::Result<(Self, JoinHandle<()>)> {
    let mut this = Self::new_(libc::CLOCK_MONOTONIC)?;
    this.setup_timeout(until)?;
    let timer_cloned = Self(this.0.try_clone().unwrap(), this.1.clone());
    let handle = std::thread::spawn(move || {
      while timer_cloned.ticks().unwrap() == 0 {}
      if let Ok(mut s) = timer_cloned.1.lock() {
        s.is_finished
          .store(true, std::sync::atomic::Ordering::Relaxed);
        if let Some(waker) = s.waker.take() {
          waker.wake();
        }
      }
    });
    Ok((this, handle))
  }

  pub fn ticks(&self) -> std::io::Result<u64> {
    loop {
      unsafe {
        let mut tmp = 0u64;
        match err_handle(libc::read(self.0.as_raw_fd(), addr_of_mut!(tmp).cast(), 8) as _) {
          Ok(_) => {
            return Ok(tmp);
          }
          Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
            return Ok(0);
          }
          Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
          Err(e) => {
            return Err(e);
          }
        }
      }
    }
  }
}

impl Future for Timer {
  type Output = u64;

  fn poll(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    let mut s = self.1.lock().unwrap();
    if s.is_finished.load(std::sync::atomic::Ordering::Relaxed) {
      std::task::Poll::Ready(0)
    } else {
      if let Some(ref w) = s.waker {
        if w.will_wake(cx.waker()) {
          return std::task::Poll::Pending;
        }
      }
      s.waker.replace(cx.waker().clone());
      std::task::Poll::Pending
    }
  }
}

impl AsRawFd for Timer {
  fn as_raw_fd(&self) -> std::os::fd::RawFd {
    self.0.as_raw_fd()
  }
}
