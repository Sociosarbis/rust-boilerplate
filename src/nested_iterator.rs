#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

struct NestedIterator {
  list: Vec<i32>,
  i: usize
}

fn resolve_nested_list(nested_list: &Vec<NestedInteger>, list: &mut Vec<i32>) {
  for item in nested_list {
    match item {
      NestedInteger::Int(v) => {
        list.push(*v)
      },
      NestedInteger::List(v) => {
        resolve_nested_list(v, list)
      } 
    }
  }
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut list = vec![];
        resolve_nested_list(&nested_list, &mut list);
        NestedIterator {
          i: 0,
          list: list
        }
    }
    #[allow(dead_code)]
    fn next(&mut self) -> i32 {
      let ret = self.list[self.i];
      self.i += 1;
      ret
    }
    
    #[allow(dead_code)]
    fn has_next(&self) -> bool {
      self.i < self.list.len()
    }
}