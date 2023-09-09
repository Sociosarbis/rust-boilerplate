use super::*;

impl Solution {
  pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
    (arrival_time + delayed_time) % 24
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    arrival_time: i32,
    delayed_time: i32,
    ret: i32,
  }

  #[test]
  fn test_find_delayed_arrival_time_simple() {
    let suites = vec![
      Suite {
        arrival_time: 15,
        delayed_time: 5,
        ret: 20,
      },
      Suite {
        arrival_time: 13,
        delayed_time: 11,
        ret: 0,
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::find_delayed_arrival_time(s.arrival_time, s.delayed_time)
      );
    }
  }
}
