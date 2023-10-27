use super::*;

impl Solution {
  pub fn max_area(
    h: i32,
    w: i32,
    mut horizontal_cuts: Vec<i32>,
    mut vertical_cuts: Vec<i32>,
  ) -> i32 {
    horizontal_cuts.insert(0, 0);
    horizontal_cuts.push(h);
    vertical_cuts.insert(0, 0);
    vertical_cuts.push(w);
    horizontal_cuts.sort_unstable();
    vertical_cuts.sort_unstable();
    let mut h_max = 0;
    for (i, &n) in horizontal_cuts.iter().enumerate().skip(1) {
      let temp = n - horizontal_cuts[i - 1];
      if temp > h_max {
        h_max = temp;
      }
    }
    let mut w_max = 0;
    for (i, &n) in vertical_cuts.iter().enumerate().skip(1) {
      let temp = n - vertical_cuts[i - 1];
      if temp > w_max {
        w_max = temp;
      }
    }
    ((h_max as i64 * w_max as i64) % 1000000007) as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    h: i32,
    w: i32,
    horizontal_cuts: Vec<i32>,
    vertical_cuts: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_max_area_simple() {
    let suites = vec![
      Suite {
        h: 5,
        w: 4,
        horizontal_cuts: vec![1, 2, 4],
        vertical_cuts: vec![1, 3],
        ret: 4,
      },
      Suite {
        h: 5,
        w: 4,
        horizontal_cuts: vec![3, 1],
        vertical_cuts: vec![1],
        ret: 6,
      },
      Suite {
        h: 5,
        w: 4,
        horizontal_cuts: vec![3],
        vertical_cuts: vec![3],
        ret: 9,
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::max_area(s.h, s.w, s.horizontal_cuts, s.vertical_cuts)
      );
    }
  }
}
