use std::{cmp::Reverse, collections::BinaryHeap};

struct DinnerPlates {
    capacity: usize,
    v: i32,
    map: Vec<Vec<i32>>,
    versions: Vec<i32>,
    left: BinaryHeap<(bool, Reverse<usize>, Reverse<i32>)>,
    right: BinaryHeap<(bool, usize, Reverse<i32>)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            v: 0,
            capacity: capacity as usize,
            versions: vec![],
            map: vec![],
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }

    fn push(&mut self, val: i32) {
        while let Some((_, Reverse(id), Reverse(mut ver))) = self.left.pop() {
            if self.versions[id] == ver {
                if self.map[id].len() < self.capacity {
                    self.map[id].push(val);
                    if self.map[id].len() == 1 {
                        ver += 1;
                        self.versions[id] = ver;
                        self.right.push((true, id, Reverse(ver)));
                    }
                    self.left.push((
                        self.map[id].len() < self.capacity,
                        Reverse(id),
                        Reverse(ver),
                    ));
                    return;
                } else {
                    self.left.push((false, Reverse(id), Reverse(ver)));
                }
                break;
            }
        }
        let mut v = Vec::with_capacity(self.capacity);
        v.push(val);
        self.versions.push(0);
        self.left
            .push((v.len() < self.capacity, Reverse(self.map.len()), Reverse(0)));
        self.right.push((v.len() > 0, self.map.len(), Reverse(0)));
        self.map.push(v);
    }

    fn pop(&mut self) -> i32 {
        while let Some((_, id, Reverse(mut ver))) = self.right.pop() {
            if self.versions[id] == ver {
                if let Some(ret) = self.map[id].pop() {
                    if self.map[id].len() + 1 == self.capacity {
                        ver += 1;
                        self.versions[id] = ver;
                        self.left.push((true, Reverse(id), Reverse(ver)));
                    }
                    self.right.push((self.map[id].len() > 0, id, Reverse(ver)));
                    return ret;
                } else {
                    self.right.push((false, id, Reverse(ver)));
                }
                break;
            }
        }
        -1
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let id = index as usize;
        if id >= self.versions.len() {
            return -1;
        }
        let ret = if let Some(v) = self.map[id].pop() {
            v
        } else {
            -1
        };
        if ret != -1 {
            let left_changed = self.map[id].len() + 1 == self.capacity;
            let right_changed = self.map[id].is_empty();
            if left_changed || right_changed {
              self.versions[id] += 1;
              self.left.push((left_changed, Reverse(id), Reverse(self.versions[id])));
              self.right.push((!right_changed, id, Reverse(self.versions[id])));
            }
        }
        ret
    }
}
