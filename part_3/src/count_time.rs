use super::*;

impl Solution {
  pub fn count_time(time: String) -> i32 {
    let chars: Vec<u8> = time.bytes().collect();
    let a = if chars[0] == b'?' {
      if chars[1] == b'?' {
        24
      } else {
        let v = chars[1] as i32 - 48;
        if v <= 3 {
          3
        } else {
          2
        }
      }
    } else if chars[1] == b'?' {
      let v = chars[0] as i32 - 48;
      if v == 2 {
        4
      } else {
        10
      }
    } else {
      1
    };
    let b = if chars[3] == b'?' {
      if chars[4] == b'?' {
        60
      } else {
        6
      }
    } else if chars[4] == b'?' {
      10
    } else {
      1
    };
    a * b
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    time: String,
    ret: i32,
  }

  #[test]
  fn test_count_time_simple() {
    let suites = vec![
      Suite {
        time: "?5:00".to_string(),
        ret: 2,
      },
      Suite {
        time: "0?:0?".to_string(),
        ret: 100,
      },
      Suite {
        time: "??:??".to_string(),
        ret: 1440,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_time(s.time));
    }
  }
}
