use super::*;

use std::collections::HashSet;

impl Solution {
  pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
    let n = favorite.len();
    let mut indeg = vec![0;n];
    // 到达节点i无环的最长路径
    let mut ma = vec![1;n];
    let mut q = vec![];
    for &f in &favorite {
      indeg[f as usize] += 1;
    }
    for (i, &count) in indeg.iter().enumerate() {
      if count == 0 {
        q.push(i as i32);
      }
    }
    while !q.is_empty() {
      let mut next_q = vec![];
      for i in q {
        let next = favorite[i as usize];
        indeg[next as usize] -= 1;
        ma[next as usize] = ma[next as usize].max(ma[i as usize] + 1);
        if indeg[next as usize] == 0 {
          next_q.push(next);
        }
      }
      q = next_q;
    } 
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut visited = HashSet::new();
    for i in 0..n {
      if indeg[i] != 0 && !visited.contains(&(i as i32)) {
        visited.insert(i as i32);
        let mut len = 1;
        let mut cur = favorite[i];
        while cur != i as i32 {
          len += 1;
          visited.insert(cur);
          cur = favorite[cur as usize];
        }
        if len > 2 {
          ans1 = ans1.max(len);
        } else {
          // 各组长度为2的环加两端无环最长路径可以合起来
          ans2 += ma[i] + ma[favorite[i] as usize];
        }
      }
    }
    ans1.max(ans2)
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  struct Suite {
    favorite: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_maxiumum_invitations_simple() {
    let suites = vec![
      Suite {
        favorite: vec![2, 2, 1, 2],
        ret: 3,
      },
      Suite {
        favorite: vec![1, 2, 0],
        ret: 3,
      },
      Suite {
        favorite: vec![3, 0, 1, 4, 1],
        ret: 4,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::maximum_invitations(s.favorite));
    }
  }
}
