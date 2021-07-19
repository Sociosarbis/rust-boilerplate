use super::Solution;


impl Solution {
  pub fn max_frequency(mut nums: Vec<i32>, mut k: i32) -> i32 {
    nums.sort();
    let mut ret = 0;
    let mut i = nums.len() - 1;
    let mut j = i;
    loop {
      let mut next_i = i;
      while next_i > 0 && nums[next_i - 1] == nums[i] {
        next_i -= 1;
      }
      if j > next_i {
        j = next_i;
      }
      while j > 0 && nums[j - 1] + k >= nums[i] {
        k -= nums[i] - nums[j - 1];
        j -= 1;
      }
      if i - j + 1 > ret {
        ret = i - j + 1;
      }
      if next_i > 0 {
        k += (nums[next_i] - nums[next_i - 1]) *  (next_i - j) as i32;
        i = next_i - 1;
      } else {
        break;
      }
    }
    ret as i32
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
  fn test_max_frequency_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,2,4],
        k: 5,
        ret: 3
      },
      Suite {
        nums: vec![1,4,8,13],
        k: 5,
        ret: 2
      },
      Suite {
        nums: vec![3,9,6],
        k: 2,
        ret: 1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_frequency(s.nums, s.k));
    }
  }
}