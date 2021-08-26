use super::*;

impl Solution {
  pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    let mut prefix_sum: Vec<i32> = (0..(right - left + 2)).collect();
    for range in ranges {
      if !(range[0] > right || range[1] < left) {
        let l = if range[0] < left { 1 } else { 1 + if range[0] > right { right - left } else { range[0] - left } } as usize;
        let r = if range[1] < left { 1 } else { 1 + if range[1] > right { right - left } else { range[1] - left } } as usize;
        if prefix_sum[r] - prefix_sum[l - 1] != 0 {
          let delta = prefix_sum[r] - prefix_sum[l - 1];
          for i in r + 1..prefix_sum.len() {
            prefix_sum[i] -= delta;
          }
          for i in l..r + 1 {
            prefix_sum[i] = prefix_sum[l - 1];
          }
        }
      }
    }
  
    *prefix_sum.last().unwrap() == 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    ranges: Vec<&'a[i32]>,
    left: i32,
    right: i32,
    ret: bool
  }

  #[test]
  fn test_is_covered_simple() {
    let suites = vec![
      Suite {
        ranges: vec![&[1,2],&[3,4],&[5,6]],
        left: 2,
        right: 5,
        ret: true
      },
      Suite {
        ranges: vec![&[1,10],&[10,20]],
        left: 21,
        right: 21,
        ret: false
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::is_covered(Solution::t2_i(s.ranges), s.left, s.right));
    }
  }
}