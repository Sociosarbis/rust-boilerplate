use super::Solution;


impl Solution {
  pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
    let mut groups: Vec<Vec<i32>> = vec![];
    let ret: Vec<i32> = vec![];
    for i in 0..group.len() {
      if group[i] != -1 {
        while group[i] as usize >= groups.len() {
          groups.push(vec![]);
        }
        groups[group[i] as usize].push(i as i32);
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    m: i32,
    group: Vec<i32>,
    before_items: Vec<Vec<i32>>,
    ret: Vec<i32>
  }

  #[test]
  fn sort_items_simple() {
    let suites: Vec<Suite> = vec![
      Suite {
        n: 8,
        m: 2,
        group: vec![-1,-1,1,0,0,1,0,-1],
        before_items: vec![vec![],vec![6],vec![5],vec![6],vec![3,6],vec![],vec![],vec![]],
        ret: vec![6,3,4,1,5,2,0,7]
      },
      Suite {
        n: 8,
        m: 2,
        group: vec![-1,-1,1,0,0,1,0,-1],
        before_items: vec![vec![],vec![6],vec![5],vec![6],vec![3],vec![],vec![4],vec![]],
        ret: vec![]
      }
    ];

    for s in suites {
      assert_eq!(Solution::sort_items(s.n, s.m, s.group, s.before_items), s.ret);
    }
  }
}