use super::*;

impl Solution {
  pub fn longest_decomposition(text: String) -> i32 {
    let chars: Vec<i64> = text.bytes().map(|item| (item - 96) as i64).collect();
    let p: i64 = 31;
    let m = 1e9 as i64 + 9;
    let mut ret = 0;
    let mut n = 0;
    let len = text.len();
    let half = len / 2;
    let mut left: i64 = 0;
    let mut right: i64 = 0;
    let mut base: i64 = 1;
    for i in 0..half {
      if n == 0 {
        if chars[i] == chars[len - 1 - i] {
          ret += 2;
        } else {
          n += 1;
          base = p;
          left += chars[i];
          right += chars[len - 1 - i];
        }
      } else {
        left = (left * p + chars[i]) % m;
        right = (right + (chars[len - i - 1] * base)) % m;
        if left == right {
          left = 0;
          right = 0;
          base = 1;
          ret += 2;
          n = 0;
        } else {
          base = (base * p) % m;
          n += 1;
        }
      }
    }
    if n != 0 {
      ret += 1;
    } else if len & 1 == 1 {
      ret += 1;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    text: String,
    ret: i32
  }

  #[test]
  fn test_longest_decomposition_simple() {
    let suites = vec![
      Suite {
        text: "ghiabcdefhelloadamhelloabcdefghi".to_string(),
        ret: 7
      },
      Suite {
        text: "merchant".to_string(),
        ret: 1
      },
      Suite {
        text: "antaprezatepzapreanta".to_string(),
        ret: 11
      },
      Suite {
        text: "aaa".to_string(),
        ret: 3
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::longest_decomposition(s.text));
    }
  }
}