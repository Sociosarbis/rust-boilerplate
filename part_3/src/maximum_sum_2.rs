use super::*;

impl Solution {
  pub fn maximum_sum_2(nums: Vec<i32>) -> i32 {
    let mut max_values = [0; 82];
    let mut ret = -1;
    for num in nums {
      let mut remain = num;
      let mut temp = 0;
      while remain != 0 {
        temp += remain % 10;
        remain /= 10;
      }
      let temp = temp as usize;
      if max_values[temp] != 0 {
        ret = ret.max(max_values[temp] + num);
        max_values[temp] = max_values[temp].max(num);
      } else {
        max_values[temp] = num;
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
    ret: i32,
  }

  #[test]
  fn test_maximum_2_simple() {
    let suites = vec![
      Suite {
        nums: vec![18, 43, 36, 13, 7],
        ret: 54,
      },
      Suite {
        nums: vec![10, 12, 19, 14],
        ret: -1,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::maximum_sum_2(s.nums));
    }
  }
}
