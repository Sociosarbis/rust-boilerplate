use std::mem::ManuallyDrop;

pub struct ScopeGuard<T, P: FnOnce(T) -> ()> {
  data: ManuallyDrop<T>,
  drop_fn: ManuallyDrop<P>,
}

impl<T, P: FnOnce(T)> ScopeGuard<T, P> {
  pub fn new(data: T, drop_fn: P) -> Self {
    Self {
      data: ManuallyDrop::new(data),
      drop_fn: ManuallyDrop::new(drop_fn),
    }
  }
}

impl<T, P: FnOnce(T)> Drop for ScopeGuard<T, P> {
  fn drop(&mut self) {
    // 因为&mut，所以不能直接用self.drop_fn(self.data)
    // (这样会产生move)，要使用ptr::read
    let (data, drop_fn) = unsafe { (std::ptr::read(&*self.data), std::ptr::read(&*self.drop_fn)) };
    drop_fn(data);
  }
}
