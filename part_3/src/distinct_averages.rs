use super::*;

impl Solution {
  pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let mut i = 0;
    let mut j = nums.len() - 1;
    let mut counter = vec![false; 201];
    while i < j {
      let temp = (nums[i] + nums[j]) as usize;
      if !counter[temp] {
        counter[temp] = true;
      }
      i += 1;
      j = j.saturating_sub(1);
    }
    counter
      .into_iter()
      .fold(0, |acc, exist| acc + if exist { 1 } else { 0 })
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
  fn test_distinct_averages_simple() {
    let suites = vec![
      Suite {
        nums: vec![4, 1, 4, 0, 3, 5],
        ret: 2,
      },
      Suite {
        nums: vec![1, 100],
        ret: 1,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::distinct_averages(s.nums));
    }
  }
}
