use super::*;

impl Solution {
  pub fn modify_string(s: String) -> String {
    let mut bytes: Vec<u8> = s.bytes().collect();
    for i in 0..bytes.len() {
      match bytes[i] {
        97..=122 => {
        },
        _ => {
          let mut b = 97;
          while (i != 0 && bytes[i - 1] == b) || (i + 1 < bytes.len() && bytes[i + 1] == b) {
            b += 1;
          }
          bytes[i] = b;
        }
      }
    }
    bytes.into_iter().map(|b| b as char).collect()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    s: &'a str,
    ret: &'a str
  }

  #[test]
  fn test_modify_string_simple() {
    let suites = vec![
      Suite {
        s: "?zs",
        ret: "azs"
      },
      Suite {
        s: "ubv?w",
        ret: "ubvaw"
      },
    ];

    for s in suites {
      assert_eq!(s.ret.to_string(), Solution::modify_string(s.s.to_string()));
    }
  }
}