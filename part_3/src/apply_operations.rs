use super::*;

impl Solution {
  pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    for i in 0..n - 1 {
      if nums[i] != 0 && nums[i] == nums[i + 1] {
        nums[i] *= 2;
        nums[i + 1] = 0;
      }
    }
    let mut j = 0;
    for i in 0..n {
      if nums[i] == 0 {
        if j <= i {
          j = i + 1;
        }
        while j < n {
          if nums[j] != 0 {
            nums.swap(i, j);
            j += 1;
            break;
          }
          j += 1;
        }
        if j >= n {
          break;
        }
      }
    }
    nums
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: Vec<i32>,
  }

  #[test]
  fn test_apply_operations_simple() {
    let suites = vec![
      Suite {
        nums: vec![1, 2, 2, 1, 1, 0],
        ret: vec![1, 4, 2, 0, 0, 0],
      },
      Suite {
        nums: vec![0, 1],
        ret: vec![1, 0],
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::apply_operations(s.nums));
    }
  }
}
