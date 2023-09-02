use super::*;

impl Solution {
  pub fn capture_forts(forts: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut temp = 0;
    let mut start = 0;
    for fort in forts {
      match fort {
        1 | -1 => {
          if start != 0 {
            if start != fort {
              if temp > ret {
                ret = temp;
              }
            }
          }
          temp = 0;
          start = fort;
        }
        _ => {
          if start != 0 {
            temp += 1;
          }
        }
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    forts: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_capture_forts_simple() {
    let suites = vec![
      Suite {
        forts: vec![1, 0, 0, -1, 0, 0, 0, 0, 1],
        ret: 4,
      },
      Suite {
        forts: vec![0, 0, 1, -1],
        ret: 0,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::capture_forts(s.forts));
    }
  }
}
