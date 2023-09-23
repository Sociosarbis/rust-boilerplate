struct LockingTree {
  parent: Vec<i32>,
  lock_state: Vec<i32>,
  children: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LockingTree {
  fn new(parent: Vec<i32>) -> Self {
    let mut children = vec![vec![]; parent.len()];
    for (i, &p) in parent.iter().enumerate().skip(1) {
      children[p as usize].push(i as i32);
    }
    Self {
      lock_state: vec![0; parent.len()],
      parent: parent,
      children: children,
    }
  }

  fn lock(&mut self, num: i32, user: i32) -> bool {
    if self.lock_state[num as usize] == 0 {
      self.lock_state[num as usize] = user;
      return true;
    }
    false
  }

  fn unlock(&mut self, num: i32, user: i32) -> bool {
    if self.lock_state[num as usize] == user {
      self.lock_state[num as usize] = 0;
      return true;
    }
    false
  }

  fn upgrade(&mut self, num: i32, user: i32) -> bool {
    if self.lock_state[num as usize] == 0 {
      let mut parent = self.parent[num as usize];
      while parent != -1 {
        if self.lock_state[parent as usize] != 0 {
          return false;
        }
        parent = self.parent[parent as usize];
      }
      let mut bfs = self.children[num as usize].clone();
      let mut ret = false;
      while !bfs.is_empty() {
        let n = bfs.len();
        for i in 0..n {
          let num = bfs[i];
          if self.lock_state[num as usize] != 0 {
            self.lock_state[num as usize] = 0;
            ret = true;
          }
          bfs.extend(self.children[num as usize].iter());
        }
        bfs.drain(0..n);
      }
      if ret {
        self.lock_state[num as usize] = user;
      }
      return ret;
    }
    false
  }
}
