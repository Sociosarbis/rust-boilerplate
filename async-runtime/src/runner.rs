use std::{future::Future, task::Context, cell::Cell};

use crate::{epoll::Epoll, event::{Event, waker}};

pub struct Runner {
  pub epoll: Epoll,
}

impl Runner {
  pub fn new() -> std::io::Result<Self> {
    Ok(Self {
      epoll: Epoll::new()?,
    })
  }

  pub fn block_on<T>(&self, fut: impl Future<Output = T>) -> std::io::Result<T> {
    let mut fut = std::pin::pin!(fut);

    let event = Event::new()?;
    let recv = (Cell::new(false), Cell::new(None));
    let _ = self.epoll.listen_read(&event, &recv)?;
    let waker = waker(event.try_clone()?);
    let mut cx = Context::from_waker(&waker);
    loop {
      match fut.as_mut().poll(&mut cx) {
        std::task::Poll::Ready(x) => return Ok(x),
        std::task::Poll::Pending => self.epoll.wait()?,
      }
      let _ = event.try_get()?;
    }
  }
}