use std::{collections::HashMap, rc::Rc, cell::RefCell};

#[derive(Debug)]
struct Node {
  count: i32,
  key: String,
  next: Option<Rc<RefCell<Node>>>,
  prev: Option<Rc<RefCell<Node>>>
}

struct AllOne {
  map: HashMap<String, Rc<RefCell<Node>>>,
  head: Option<Rc<RefCell<Node>>>,
  tail: Option<Rc<RefCell<Node>>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {

  fn new() -> Self {
    return AllOne {
      map: HashMap::new(),
      head: None,
      tail: None
    }
  }
  
  fn inc(&mut self, key: String) {
    if let Some(item) = self.map.get(&key) {
      item.borrow_mut().count += 1;
      let count = item.borrow().count;
      let mut is_same = true;
      let mut node_1 = item.clone();
      while let Some(ref node) = node_1.clone().borrow().prev {
        if node.borrow().count >= count {
          break;
        } else {
          if is_same {
            is_same = false;
          }
        }
        node_1 = node.clone();
      }
      if !is_same {
        
        if node_1.borrow().count == count {
          let node_1_next = node_1.borrow().next.clone();
          let node_2_prev = item.borrow().prev.clone();
          let node_2_next = item.borrow().next.clone();
          node_1.borrow_mut().next = Some(item.clone());
          if let Some(node) = &node_1_next {
            node.borrow_mut().prev = Some(item.clone());
          }
          item.borrow_mut().next = node_1_next;
          item.borrow_mut().prev = Some(node_1.clone());
          if let Some(node) = &node_2_next {
            node.borrow_mut().prev = node_2_prev.clone();
          } else {
            self.tail = node_2_prev.clone();
          }
          node_2_prev.unwrap().borrow_mut().next = node_2_next;
        } else {
          let node_2_prev = item.borrow().prev.clone();
          let node_2_next = item.borrow().next.clone();
          if let Some(node) = &self.head {
            node.borrow_mut().prev = Some(item.clone());
          }
          item.borrow_mut().prev = None;
          item.borrow_mut().next = self.head.clone();
          self.head = Some(item.clone());
          if let Some(node) = &node_2_next {
            node.borrow_mut().prev = node_2_prev.clone();
          } else {
            self.tail = node_2_prev.clone();
          }
          node_2_prev.unwrap().borrow_mut().next = node_2_next;
        }
      }
    } else {
      let node = Rc::new(RefCell::new(Node {
        count: 1,
        key: key.clone(),
        prev: None,
        next: None
      }));
      self.map.insert(key, node.clone());
      if let Some(t) = &self.tail.clone() {
        node.borrow_mut().prev = Some(t.clone());
        self.tail = Some(node);
        t.borrow_mut().next = self.tail.clone();
      } else {
        self.head = Some(node);
        self.tail = self.head.clone();
      }
    }
  }

  fn dec(&mut self, key: String) {
    if let Some(item) = self.map.get(&key) {
      item.borrow_mut().count -= 1;
      let count = item.borrow().count;
      let mut is_same = true;
      let mut node_1 = item.clone();
      while let Some(ref node) = node_1.clone().borrow().next {
        if node.borrow().count <= count {
          break;
        } else {
          if is_same {
            is_same = false;
          }
        }
        node_1 = node.clone();
      }
      if !is_same {
        if node_1.borrow().count == count {
          let node_1_prev = node_1.borrow().prev.clone();
          let node_2_prev = item.borrow().prev.clone();
          let node_2_next = item.borrow().next.clone();
          node_1.borrow_mut().prev = Some(item.clone());
          if let Some(node) = &node_1_prev {
            node.borrow_mut().next = Some(item.clone());
          }
          item.borrow_mut().prev = node_1_prev;
          item.borrow_mut().next = Some(node_1.clone());
          if let Some(node) = &node_2_prev {
            node.borrow_mut().next = node_2_next.clone();
          } else {
            self.head = node_2_next.clone();
          }
          node_2_next.unwrap().borrow_mut().prev = node_2_prev;
        } else {
          let node_2_prev = item.borrow().prev.clone();
          let node_2_next = item.borrow().next.clone();
          if let Some(node) = &self.tail {
            node.borrow_mut().next = Some(item.clone());
          }
          item.borrow_mut().next = None;
          item.borrow_mut().prev = self.tail.clone();
          self.tail = Some(item.clone());
          if let Some(node) = &node_2_prev {
            node.borrow_mut().next = node_2_next.clone();
          } else {
            self.head = node_2_next.clone();
          }
          node_2_next.unwrap().borrow_mut().prev = node_2_prev;
        }
      }

      if count == 0 {
        if let Some(node) = &self.tail.clone() {
          self.tail = node.borrow_mut().prev.clone();
        }

        if let Some(node) = &self.tail {
          node.borrow_mut().next = None;
        } else {
          self.head = None;
        }
        self.map.remove(&key);
      }
    }
  }

  fn get_max_key(&self) -> String {
    if let Some(node) = &self.head {
      node.borrow().key.clone()
    } else {
      String::new()
    }
  }
  
  fn get_min_key(&self) -> String {
    if let Some(node) = &self.tail {
      node.borrow().key.clone()
    } else {
      String::new()
    }
  }
}