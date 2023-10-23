use super::*;

impl Solution {
  pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
    satisfaction.sort_unstable();
    let mut l = satisfaction.len();
    let mut ret = 0;
    while l > 0 && satisfaction[l - 1] + ret > 0 {
      l -= 1;
      ret += satisfaction[l];
    }
    satisfaction
      .into_iter()
      .skip(l)
      .enumerate()
      .fold(0, |acc, (i, num)| acc + (i + 1) as i32 * num)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    satisfaction: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_max_satisfaction_simple() {
    let suites = vec![
      Suite {
        satisfaction: vec![-1, -8, 0, 5, -9],
        ret: 14,
      },
      Suite {
        satisfaction: vec![4, 3, 2],
        ret: 20,
      },
      Suite {
        satisfaction: vec![-5,-7,8,-2,1,3,9,5,-10,4,-5,-2,-1,-6],
        ret: 260
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_satisfaction(s.satisfaction));
    }
  }
}
