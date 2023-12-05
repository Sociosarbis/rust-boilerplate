use std::{
  cell::Cell,
  future::poll_fn,
  task::{Context, Waker},
};

use crate::epoll::Epoll;



pub struct PollEvent<'a, 'b> {
  fd: libc::c_int,
  recv: &'b (Cell<bool>, Cell<Option<Waker>>),
  poll: &'a Epoll,
}

impl<'a, 'b> PollEvent<'a, 'b> {
  pub fn new(
    fd: libc::c_int,
    recv: &'b (Cell<bool>, Cell<Option<Waker>>),
    poll: &'a Epoll,
  ) -> Self {
    Self { fd, recv, poll }
  }

  pub async fn wait(&mut self) {
    poll_fn(|cx| self.poll_wait(cx)).await
  }

  fn poll_wait(&mut self, cx: &mut Context<'_>) -> std::task::Poll<()> {
    if self.recv.0.replace(false) {
      std::task::Poll::Ready(())
    } else {
      if let Some(prev) = self.recv.1.take() {
        if prev.will_wake(cx.waker()) {
          self.recv.1.set(Some(prev));
          return std::task::Poll::Pending;
        }
      }
      self.recv.1.set(Some(cx.waker().clone()));
      std::task::Poll::Pending
    }
  }
}

impl<'a, 'b> Drop for PollEvent<'a, 'b> {
  fn drop(&mut self) {
    self.poll.unlisten(self.fd).unwrap();
  }
}
