use super::*;

impl Solution {
  pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
    let mut prev_seat: Option<usize> = None;
    let mut ret = 0;
    let n = seats.len();
    for (i, s) in seats.into_iter().enumerate() {
      if s == 1 {
        if let Some(p_s) = prev_seat.as_mut() {
          let mut count = i - *p_s - 1;
          count = if count & 1 == 1 {
            ((count - 1) >> 1) + 1
          } else {
            count >> 1
          };
          if count > ret {
            ret = count;
          }
          *p_s = i;
        } else {
          ret = i;
          prev_seat = Some(i);
        }
      }
    }
    if let Some(p_s) = prev_seat.take() {
      let count = n - p_s - 1;
      if count > ret {
        ret = count;
      }
    }
    ret as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    seats: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_max_dist_to_closest_simple() {
    let suites = vec![
      Suite {
        seats: vec![1, 0, 0, 0, 1, 0, 1],
        ret: 2,
      },
      Suite {
        seats: vec![1, 0, 0, 0],
        ret: 3,
      },
      Suite {
        seats: vec![0, 1],
        ret: 1,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_dist_to_closest(s.seats));
    }
  }
}
