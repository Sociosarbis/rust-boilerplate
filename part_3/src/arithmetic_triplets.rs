use super::*;

impl Solution {
  pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
    let mut counter = vec![0;201];
    let mut counter_2 = vec![0;201];
    let mut ret = 0;
    for num in nums {
      counter[num as usize] += 1;
      if num >= diff {
        let prev = num - diff;
        let prev_count = counter[prev as usize];
        counter_2[num as usize] += prev_count;
        ret += counter_2[prev as usize];
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
    diff: i32,
    ret: i32,
  }

  #[test]
  fn test_arithmetic_triplets_simple() {
    let suites = vec![
      Suite {
        nums: vec![0,1,4,6,7,10],
        diff: 3,
        ret: 2
      },
      Suite {
        nums: vec![4,5,6,7,8,9],
        diff: 2,
        ret: 2
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::arithmetic_triplets(s.nums, s.diff));
    }
  }
}

