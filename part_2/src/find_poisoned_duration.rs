use super::*;

impl Solution {
  pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    let mut end_time = -1;
    let mut ret = -duration;
    for t in time_series {
      if t > end_time {
        ret += duration;
      } else {
        ret += t - (end_time - duration + 1);
      }
      end_time = t + duration - 1;
    }
    ret + duration
  }
}

#[cfg(test)]
mod tests {
  use super::*;


  struct Suite {
    time_series: Vec<i32>,
    duration: i32,
    ret: i32
  }

  #[test]
  fn test_find_poisoned_duration_simple() {
    let suites = vec![
      Suite {
        time_series: vec![1,4],
        duration: 2,
        ret: 4
      },
      Suite {
        time_series: vec![1,2,2],
        duration: 2,
        ret: 3
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_poisoned_duration(s.time_series, s.duration));
    }
  }
}
