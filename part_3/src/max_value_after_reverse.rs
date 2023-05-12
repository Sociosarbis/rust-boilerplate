use super::*;

impl Solution {
  pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
    let mut min_max = 0;
    let mut max_min = 200000;
    let mut total = 0;
    let mut temp = 0;
    let last_index = nums.len() - 1;
    // a,b 和 c,d表示两对相邻的元素，假设b < c，需要最大化min(c, d) - max(a, b)的值
    for (i, &num) in nums.iter().enumerate().take(last_index) {
      let diff = (nums[i + 1] - num).abs();
      total += diff;
      // 处理0开始或者last_index结尾的交互的情况
      temp = temp.max((nums[0] - nums[i + 1]).abs() - diff).max((nums[last_index] - num).abs() - diff);
      max_min = max_min.min(num.max(nums[i + 1]));
      min_max = min_max.max(num.min(nums[i + 1]));
    }
    total + temp.max((min_max - max_min) * 2)
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_max_value_after_reverse_simple() {
    let suites = vec![
      Suite {
        nums: vec![2,3,1,5,4],
        ret: 10,
      },
      Suite {
        nums: vec![2,4,9,24,2,1,10],
        ret: 68
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_value_after_reverse(s.nums));
    }
  }
}