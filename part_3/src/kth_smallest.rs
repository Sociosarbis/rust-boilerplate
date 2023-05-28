use super::*;

use std::{cmp::Reverse, collections::BinaryHeap, collections::HashSet};

impl Solution {
  pub fn kth_smallest(mat: Vec<Vec<i32>>, mut k: i32) -> i32 {
    let mut ret = 0;
    let m = mat.len();
    let n = mat[0].len();
    for row in &mat {
      ret += row[0];
    }
    if k != 1 {
      let mut visited: HashSet<Vec<usize>> = HashSet::new();
      visited.insert(vec![0; mat.len()]);
      let mut queue: BinaryHeap<(Reverse<i32>, Vec<usize>)> = BinaryHeap::new();
      queue.push((Reverse(ret), vec![0; mat.len()]));
      while k != 0 {
        if let Some((Reverse(v), mut indices)) = queue.pop() {
          ret = v;
          for i in 0..m {
            if indices[i] + 1 < n {
              indices[i] += 1;
              if !visited.contains(&indices) {
                visited.insert(indices.clone());
                queue.push((
                  Reverse(v + mat[i][indices[i]] - mat[i][indices[i] - 1]),
                  indices.clone(),
                ));
              }
              indices[i] -= 1;
            }
          }
        }
        k -= 1;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    mat: Vec<Vec<i32>>,
    k: i32,
    ret: i32,
  }

  #[test]
  fn test_kth_smallest_simple() {
    let suites = vec![
      Suite {
        mat: t2_i![[1, 3, 11], [2, 4, 6]],
        k: 5,
        ret: 7,
      },
      Suite {
        mat: t2_i![[1, 3, 11], [2, 4, 6]],
        k: 9,
        ret: 17,
      },
      Suite {
        mat: t2_i![[1, 10, 10], [1, 4, 5], [2, 3, 6]],
        k: 7,
        ret: 9,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::kth_smallest(s.mat, s.k));
    }
  }
}
