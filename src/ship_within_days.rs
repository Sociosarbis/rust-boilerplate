use super::Solution;

impl Solution {
  pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
    let n = weights.len() as i32;
    let mut left = n / d;
    let mut right = 500 * n / d;
    // 二分搜索符合条件的载重量
    while left <= right {
      let mut temp = 0;
      let mut temp_d = 1;
      let mid  = (left + right) / 2;
      for i in 0..n as usize {
        if temp + weights[i] <= mid {
          temp += weights[i];
        } else if weights[i] > mid {
          left = mid + 1;
          break;
        } else {
          temp_d += 1;
          temp = weights[i];
        }
        if temp_d > d {
          left = mid + 1;
          break;
        }

        if i == n as usize - 1 && temp_d <= d {
          right = mid - 1;
        }
      }
    }
    left as i32
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    weights: Vec<i32>,
    d: i32,
    ret: i32
  }

  #[test]
  fn test_ship_within_days_simple() {
    let suites = vec![
      Suite {
        weights: vec![1,2,3,4,5,6,7,8,9,10],
        d: 5,
        ret: 15
      },
      Suite {
        weights: vec![3,2,2,4,1,4],
        d: 3,
        ret: 6
      },
      Suite {
        weights: vec![1,2,3,1,1],
        d: 4,
        ret: 3
      }
    ];

    for s in suites {
      assert_eq!(Solution::ship_within_days(s.weights, s.d), s.ret);
    }
  }
}