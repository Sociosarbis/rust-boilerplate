use super::*;

impl Solution {
  pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let mut prefix_sums = vec![0;n as usize];
    for booking in &bookings {
      for i in booking[0] - 1..booking[1] {
        prefix_sums[i as usize] += booking[2];
      }
    }
    prefix_sums
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    bookings: Vec<Vec<i32>>,
    n: i32,
    ret: Vec<i32>
  }

  #[test]
  fn test_corp_flight_bookings_simple() {
    let suites = vec![
      Suite {
        bookings: t2_i![[1,2,10],[2,3,20],[2,5,25]],
        n: 5,
        ret: vec![10,55,45,25,25]
      },
      Suite {
        bookings: t2_i![[1,2,10],[2,2,15]],
        n: 2,
        ret: vec![10,25]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::corp_flight_bookings(s.bookings, s.n));
    }
  }
}