use super::*;

impl Solution {
  pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
    satisfaction.sort_unstable();
    let mut l = 0;
    let mut r = satisfaction.len() - 1;
    while l <= r {
      let mid = (l + r) >> 1;
      if satisfaction[mid] > 0 {
        if mid > 0 && satisfaction[mid - 1] > 0 {
          r = mid - 1;
        } else {
          l = mid;
          break;
        }
      } else {
        l = mid + 1;
      }
    }
    let mut positive_sum = 0;
    for i in l..satisfaction.len() {
      positive_sum += satisfaction[i];
    }
    r = l.min(satisfaction.len() - 1);
    l = 0;
    while l <= r {
      let mid = (l + r) >> 1;
      if satisfaction[mid] > -positive_sum {
        if mid > 0 && satisfaction[mid - 1] > -positive_sum {
          r = mid - 1;
        } else {
          l = mid;
          break;
        }
      } else {
        l = mid + 1;
      }
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
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_satisfaction(s.satisfaction));
    }
  }
}
