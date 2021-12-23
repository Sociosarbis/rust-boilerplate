use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn longest_dup_substring(s: String) -> String {
    let mut l = 1;
    let mut r = s.len() - 1;
    let mut ret = String::new();
    let c = 48271;
    let m = 2147483648;
    let chars: Vec<char> = s.chars().collect();
    while l <= r {
      let mid = (l + r) >> 1;
      let mut base: u64 = 1;
      for _ in 0..mid {
        base = (base * c) % m;
      }
      let mut map: HashMap<u64, Vec<usize>> = HashMap::new();
      let mut hash = (0..mid).fold(0, |acc, i| (acc * c + chars[i] as u64) % m);
      map.insert(hash, vec![0]);
      for i in 1..s.len() - mid + 1 {
        hash = (hash * c - base * chars[i - 1] as u64 + chars[i + mid - 1] as u64) % m;
        if let Some(v) = map.get_mut(&hash) {
          let mut found = true;
          for item in v.iter() {
            for j in 0..mid {
              if chars[item + j] != chars[i + j] {
                found = false;
                break
              }
            }
            if found {
              break
            }
          }
          if found {
            ret = (i..i+mid).map(|index| chars[index]).collect();
            break
          } else {
            v.push(i);
          }
        } else {
          map.insert(hash, vec![i]);
        }
      }
      if ret.len() == mid {
        l = mid + 1;
      } else {
        if mid > 0 {
          r = mid - 1;
        } else {
          break;
        }
      }
    }
    ret
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
  fn test_longest_dup_substring_simple() {
    let suites = vec![
      Suite {
        s: "banana",
        ret: "ana"
      },
      Suite {
        s: "abcd",
        ret: ""
      }
    ];

    for s in suites {
      assert_eq!(s.ret.to_string(), Solution::longest_dup_substring(s.s.to_owned()));
    }
  }
}
