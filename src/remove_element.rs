use super::Solution;

impl Solution {
  pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    for j in 0..nums.len() {
      if nums[j] != val {
        if i != j {
          nums[i] = nums[j];
        }
        i += 1;
      }
    }
    i as i32
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    val: i32,
    ret: i32
  }

  #[test]
  fn test_remove_element() {
    let suites = vec![
      Suite {
        nums: vec![3,2,2,3],
        val: 3,
        ret: 2
      },
      Suite {
        nums: vec![0,1,2,2,3,0,4,2],
        val: 2,
        ret: 5
      },
    ];

    for mut s in suites {
      assert_eq!(Solution::remove_element(&mut s.nums, s.val), s.ret);
    }
  }
}