use super::*;

impl Solution {
  pub fn sum_of_power(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut ret = 0;
    let mut temp: i64 = 0;
    let m = 1e9 as i64 + 7;
    for num in nums {
      let num_2 = (num as i64).pow(2) % m;
      ret = (ret + (temp * num_2) % m + (num_2 * num as i64) % m) % m;
      temp = ((temp * 2) % m + num as i64) % m;
    }
    ret as i32
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
  fn test_sum_of_power_simple() {
    let suites = vec![
      Suite {
        nums: vec![2, 1, 4],
        ret: 141,
      },
      Suite {
        nums: vec![1, 1, 1],
        ret: 7,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::sum_of_power(s.nums));
    }
  }
}
