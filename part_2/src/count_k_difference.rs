use super::*;

impl Solution {
  pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut counter = vec![0;100];
    let mut ret = 0;
    for i in 0..nums.len() {
      counter[nums[i] as usize - 1] += 1;
    }
    for i in 0..100 - k as usize {
      if counter[i] != 0 && counter[i + k as usize] != 0 {
        ret += counter[i] * counter[i + k as usize];
      }
    }
    ret
  }
}



#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    k: i32,
    ret: i32
  }

  #[test]
  fn test_count_k_difference_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,2,2,1],
        k: 1,
        ret: 4
      },
      Suite {
        nums: vec![1,3],
        k: 3,
        ret: 0
      },
      Suite {
        nums: vec![3,2,1,5,4],
        k: 2,
        ret: 3
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_k_difference(s.nums, s.k));
    }
  }
}