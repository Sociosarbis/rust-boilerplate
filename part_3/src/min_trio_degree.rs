use super::*;

use std::collections::HashSet;

impl Solution {
  pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut nums = vec![HashSet::new(); n as usize + 1];
    for edge in edges {
      let a = edge[0];
      let b = edge[1];
      nums[a as usize].insert(b);
      nums[b as usize].insert(a);
    }
    let mut ret = -1;
    for i in 1..=n {
      for &j in &nums[i as usize] {
        if j > i {
          for &k in &nums[j as usize] {
            if k > i && nums[k as usize].contains(&i) {
              let temp = (nums[i as usize].len() + nums[j as usize].len() + nums[k as usize].len()
                - 6) as i32;
              if ret == -1 || temp < ret {
                ret = temp;
              }
            }
          }
        }
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
    edges: Vec<Vec<i32>>,
    ret: i32,
  }

  #[test]
  fn test_min_trio_degree_simple() {
    let suites = vec![
      Suite {
        n: 6,
        edges: t2_i![[1, 2], [1, 3], [3, 2], [4, 1], [5, 2], [3, 6]],
        ret: 3,
      },
      Suite {
        n: 7,
        edges: t2_i![
          [1, 3],
          [4, 1],
          [4, 3],
          [2, 5],
          [5, 6],
          [6, 7],
          [7, 5],
          [2, 6]
        ],
        ret: 0,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_trio_degree(s.n, s.edges));
    }
  }
}
