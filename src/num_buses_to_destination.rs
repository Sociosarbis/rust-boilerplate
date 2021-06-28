use super::Solution;

impl Solution {
  pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {

  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    routes: Vec<&'a [i32]>,
    source: i32,
    target: i32,
    ret: i32
  }

  #[test]
  fn test_num_buses_to_destination_simple() {
    let suites = vec![
      Suite {
        routes: vec![&[1,2,7],&[3,6,7]],
        source: 1,
        target: 6,
        ret: 2
      },
      Suite {
        routes: vec![&[7,12],&[4,5,15],&[6],&[15,19],&[9,12,13]],
        source: 15,
        target: 12,
        ret: -1
      }
    ];

    for s in suites {
      assert_eq!(Solution::num_buses_to_destination(Solution::t2_i(s.routes), s.source, s.target), s.ret);
    }
  }
}