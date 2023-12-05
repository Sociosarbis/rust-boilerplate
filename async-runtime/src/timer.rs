extern crate libc;
use libc::*;
use std::{
  io::ErrorKind,
  os::fd::{AsRawFd, FromRawFd, OwnedFd},
  ptr::addr_of_mut,
  time::Duration,
};

use crate::utils::err_handle;

fn dur2timespec_saturating(delta: Duration) -> libc::timespec {
  libc::timespec {
    tv_sec: delta.as_secs() as _,
    tv_nsec: delta.subsec_nanos() as _,
  }
}

#[derive(Debug)]
pub struct Timer(OwnedFd);

impl Timer {
  fn new_(clockid: libc::c_int) -> std::io::Result<Self> {
    let fd = unsafe {
      err_handle(timerfd_create(
        clockid,
        libc::TFD_NONBLOCK | libc::TFD_CLOEXEC,
      ))?
    };
    unsafe { Ok(Self(OwnedFd::from_raw_fd(fd))) }
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

  pub fn new_timeout(until: Duration) -> std::io::Result<Self> {
    let mut this = Self::new_(libc::CLOCK_MONOTONIC)?;
    this.setup_timeout(until)?;
    Ok(this)
  }

  pub fn ticks(&self) -> std::io::Result<u64> {
    loop {
      unsafe {
        let mut tmp = 0u64;
        match err_handle(libc::read(self.0.as_raw_fd(), addr_of_mut!(tmp).cast(), 8) as libc::c_int)
        {
          Ok(_) => return Ok(tmp),
          Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
            return Ok(0);
          }
          Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
          Err(e) => return Err(e),
        }
      }
    }
  }
}

impl AsRawFd for Timer {
  fn as_raw_fd(&self) -> std::os::fd::RawFd {
    self.0.as_raw_fd()
  }
}
