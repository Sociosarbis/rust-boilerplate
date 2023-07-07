use super::*;

impl Solution {
  pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut r = numbers.len() - 1;
    let mut i = 0;
    while i < r {
      let mut l = i + 1;
      while l <= r {
        let mid = (l + r) / 2;
        if numbers[i] + numbers[mid] < target {
          l = mid + 1;
        } else if numbers[i] + numbers[mid] > target {
          if mid > l && numbers[i] + numbers[mid - 1] >= target {
            r = mid - 1;
          } else {
            r = mid;
            break;
          }
        } else {
          return vec![i as i32 + 1, mid as i32 + 1];
        }
      }
      i += 1;
    }
    vec![]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    numbers: Vec<i32>,
    target: i32,
    ret: Vec<i32>,
  }

  #[test]
  fn test_two_sum_simple() {
    let suites = vec![
      Suite {
        numbers: vec![2, 7, 11, 15],
        target: 9,
        ret: vec![1, 2],
      },
      Suite {
        numbers: vec![2, 3, 4],
        target: 6,
        ret: vec![1, 3],
      },
      Suite {
        numbers: vec![-1, 0],
        target: -1,
        ret: vec![1, 2],
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::two_sum(s.numbers, s.target));
    }
  }
}
