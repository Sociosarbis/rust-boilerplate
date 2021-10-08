use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    if s.len() <= 10 {
      return Vec::new();
    }
    let mut map = HashMap::new();
    let mut mask = 0;
    let mut ret = vec![];
    let chars: Vec<char> = s.chars().collect();
    for i in 0..10 {
      mask |= (match chars[i] {
        'A' => 0,
        'C' => 1,
        'G' => 2,
        _ => 3
      }) << i * 2
    }
    map.insert(mask, true);
    for i in 10..chars.len() {
      mask = (mask >> 2) | (match chars[i] {
        'A' => 0,
        'C' => 1,
        'G' => 2,
        _ => 3
      }) << 18;
      if let Some(&res) = map.get(&mask) {
        if res {
          ret.push(chars[i-9..i+1].iter().collect());
          *map.get_mut(&mask).unwrap() = false
        }
      } else {
        map.insert(mask, true);
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
    ret: Vec<String>
  }

  #[test]
  fn test_find_repeated_dna_sequences_simple() {
    let suites = vec![
      Suite {
        s: "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT",
        ret: t1!["AAAAACCCCC","CCCCCAAAAA"]
      },
      Suite {
        s: "AAAAAAAAAAAAA",
        ret: t1!["AAAAAAAAAA"]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_repeated_dna_sequences(s.s.to_owned()));
    }
  }
}