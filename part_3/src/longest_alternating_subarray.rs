use super::*;

impl Solution {
  pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
    let mut state: u8 = 0;
    let mut ret = 0;
    let mut temp = 0;
    for num in nums {
      if num > threshold {
        if state != 0 {
          if temp > ret {
            ret = temp;
          }
          state = 0;
        }
      } else {
        match state {
          0 => {
            if num % 2 == 0 {
              temp = 1;
              state = 2;
            }
          }
          s @ _ => {
            let next_state = if num % 2 == 0 { 2 } else { 1 };
            if next_state != s {
              temp += 1;
              state = next_state;
            } else {
              if temp > ret {
                ret = temp;
              }
              if s == 1 {
                state = 0;
              } else {
                temp = 1;
              }
            }
          }
        }
      }
    }
    if state != 0 {
      if temp > ret {
        ret = temp;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    threshold: i32,
    ret: i32,
  }

  #[test]
  fn test_longest_alternating_subarray_simple() {
    let suites = vec![
      Suite {
        nums: vec![3, 2, 5, 4],
        threshold: 5,
        ret: 3,
      },
      Suite {
        nums: vec![1, 2],
        threshold: 2,
        ret: 1,
      },
      Suite {
        nums: vec![2, 3, 4, 5],
        threshold: 4,
        ret: 3,
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::longest_alternating_subarray(s.nums, s.threshold)
      );
    }
  }
}
