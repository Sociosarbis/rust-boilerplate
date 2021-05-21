use super::Solution;


impl Solution {
  pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut cache = vec![vec![-1;nums2.len()];nums1.len()];
    Solution::max_uncrossed_lines_dfs(&nums1, &nums2, 0, 0, &mut cache)
  }

  fn max_uncrossed_lines_dfs(nums1: &Vec<i32>, nums2: &Vec<i32>, i: usize, j: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
    if i < nums1.len() && j < nums2.len() {
      let mut ret = 0;
      if cache[i][j] != -1 {
        return cache[i][j];
      }
      let num1 = nums1[i];
      let mut index = j + 1;
      for k in j..nums2.len() {
        if num1 == nums2[k] {
          ret = 1 + Solution::max_uncrossed_lines_dfs(nums1, nums2, i + 1, k + 1, cache);
          index = k;
          break
        }
      }

      if index >= j + 1 {
        let option = Solution::max_uncrossed_lines_dfs(nums1, nums2, i + 1, j, cache);
        if option > ret {
          ret = option;
        }
      }
      cache[i][j] = ret;
      return ret;
    }
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_max_uncrossed_lines_simple() {
    let suites = vec![
      Suite {
        nums1: vec![1,4,2],
        nums2: vec![1,2,4],
        ret: 2
      },
      /*Suite {
        nums1: vec![2,5,1,2,5],
        nums2: vec![10,5,2,1,5,2],
        ret: 3
      },
      Suite {
        nums1: vec![1,3,7,1,7,5],
        nums2: vec![1,9,2,5,1],
        ret: 2
      },
      Suite {
        nums1: vec![3,1,2,1,4,1,2,2,5,3,2,1,1,4,5,2,3,2,5,5],
        nums2: vec![2,4,1,2,3,4,2,4,5,5,1,1,2,1,1,1,5,4,1,4,2,1,5,4,2,3,1,5,2,1],
        ret: 14
      },
      Suite {
        nums1: vec![15,14,1,7,15,1,12,18,9,15,1,20,18,15,16,18,11,8,11,18,11,11,17,20,16,20,15,15,9,18,16,4,16,1,13,10,10,20,4,18,17,3,8,1,8,19,14,10,10,12],
        nums2: vec![12,8,17,4,2,18,16,10,11,12,7,1,8,16,4,14,12,18,18,19,19,1,11,18,1,6,12,17,6,19,10,5,11,16,6,17,12,1,9,3,19,2,18,18,2,4,11,11,14,9,20,19,2,20,9,15,8,7,8,6,19,12,4,11,18,18,1,6,9,17,13,19,5,4,14,9,11,15,2,5,4,1,10,11,6,4,9,7,11,7,3,8,11,12,4,19,12,17,14,18],
        ret: 23
      }*/
    ];

    for s in suites {
      assert_eq!(Solution::max_uncrossed_lines(s.nums1, s.nums2), s.ret);
    }
  }
}