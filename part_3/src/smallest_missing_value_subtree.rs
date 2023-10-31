use super::*;

use std::{collections::HashSet, mem::swap};

impl Solution {
  pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
    let mut children = vec![vec![]; parents.len()];
    let mut ret = vec![0; parents.len()];
    for (i, p) in parents.into_iter().enumerate().skip(1) {
      children[p as usize].push(i);
    }
    Solution::smallest_missing_value_subtree_dfs(&children, 0, &nums, &mut ret);
    ret
  }

  fn smallest_missing_value_subtree_dfs(
    children: &Vec<Vec<usize>>,
    i: usize,
    nums: &Vec<i32>,
    out: &mut Vec<i32>,
  ) -> (HashSet<i32>, i32) {
    let mut ret = (HashSet::new(), 0);
    ret.0.insert(nums[i]);
    ret.1 = if nums[i] == 1 { 2 } else { 1 };
    ret = children[i].iter().fold(ret, |mut acc, &child_i| {
      let mut child_res =
        Solution::smallest_missing_value_subtree_dfs(children, child_i, nums, out);
      acc.1 = acc.1.max(child_res.1);
      if child_res.0.len() > acc.0.len() {
        swap(&mut child_res.0, &mut acc.0);
      }
      acc.0.extend(child_res.0.into_iter());
      acc
    });
    for j in ret.1..=100001 {
      if !ret.0.contains(&j) {
        ret.1 = j;
        break;
      }
    }
    out[i] = ret.1;
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    parents: Vec<i32>,
    nums: Vec<i32>,
    ret: Vec<i32>,
  }

  #[test]
  fn test_smallest_missing_value_subtree_simple() {
    let suites = vec![
      Suite {
        parents: vec![-1, 0, 0, 2],
        nums: vec![1, 2, 3, 4],
        ret: vec![5, 1, 1, 1],
      },
      Suite {
        parents: vec![-1, 0, 1, 0, 3, 3],
        nums: vec![5, 4, 6, 2, 1, 3],
        ret: vec![7, 1, 1, 4, 2, 1],
      },
      Suite {
        parents: vec![-1, 2, 3, 0, 2, 4, 1],
        nums: vec![2, 3, 4, 5, 6, 7, 8],
        ret: vec![1, 1, 1, 1, 1, 1, 1],
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::smallest_missing_value_subtree(s.parents, s.nums)
      );
    }
  }
}
