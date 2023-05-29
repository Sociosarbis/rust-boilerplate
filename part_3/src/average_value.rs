use super::*;

impl Solution {
  pub fn average_value(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut count = 0;
    for num in nums {
      if num % 6 == 0 {
        sum += num;
        count += 1;
      }
    }
    if count == 0 {
      0
    } else {
      sum / count
    }
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
  fn test_average_value_simple() {
    let suites = vec![
      Suite {
        nums: vec![1, 3, 6, 10, 12, 15],
        ret: 9,
      },
      Suite {
        nums: vec![1, 2, 4, 7, 10],
        ret: 0,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::average_value(s.nums));
    }
  }
}
