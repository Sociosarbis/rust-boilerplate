struct FrontMiddleBackQueue {
  arr: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {
  fn new() -> Self {
    Self { arr: vec![] }
  }

  fn push_front(&mut self, val: i32) {
    if self.arr.is_empty() {
      self.push_back(val);
    } else {
      self.arr.insert(0, val);
    }
  }

  fn push_middle(&mut self, val: i32) {
    if self.arr.is_empty() {
      self.push_back(val);
    } else {
      self.arr.insert(self.arr.len() / 2, val);
    }
  }

  fn push_back(&mut self, val: i32) {
    self.arr.push(val);
  }

  fn pop_front(&mut self) -> i32 {
    if self.arr.is_empty() {
      -1
    } else {
      self.arr.remove(0)
    }
  }

  fn pop_middle(&mut self) -> i32 {
    if self.arr.is_empty() {
      -1
    } else {
      self.arr.remove((self.arr.len() - 1) / 2)
    }
  }

  fn pop_back(&mut self) -> i32 {
    if self.arr.is_empty() {
      -1
    } else {
      self.arr.pop().unwrap()
    }
  }
}
