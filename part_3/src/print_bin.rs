use super::*;

impl Solution {
  pub fn print_bin(mut num: f64) -> String {
    for i in 0..32 {
      if (num as i32) as f64 == num {
        let mut num = num as i32;
        let mut bytes = vec!['0';i];
        let mut ret = "0.".to_string();
        let mut j = bytes.len();
        while num != 0 {
          if num & 1 == 1 {
            bytes[j - 1] = '1';
          }
          num >>= 1;
          j -= 1;
        }
        for c in bytes {
          ret.push(c);
        }
        return ret;
      }
      num *= 2.0;
    }
    "ERROR".to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    num: f64,
    ret: String
  }

  #[test]
  fn test_print_bin_simple() {
    let suites = vec![
      Suite {
        num: 0.625,
        ret: "0.101".to_string(),
      },
      Suite {
        num: 0.1,
        ret: "ERROR".to_string()
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::print_bin(s.num));
    }
  }
}