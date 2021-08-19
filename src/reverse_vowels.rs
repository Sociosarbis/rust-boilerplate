use super::Solution;

impl Solution {
  pub fn reverse_vowels(s: String) -> String {
    let mut i = 0;
    let mut j = s.len() - 1;
    let mut chars: Vec<char> = s.chars().collect();
    
    let mut map: [bool;128] = [false;128];
    for &k in &['a','e','i','o','u','A','E','I','O','U'] {
      map[k as usize] = true;
    }
    while i < j {
      while i < chars.len() {
        if map[chars[i] as usize] {
          break;
        }
        i += 1;
      }

      loop {
        if j == 0 || map[chars[j] as usize] {
          break;
        }
        j -= 1;
      }
      if i < j {
        chars.swap(i, j);
        i += 1;
        if j > 0 {
          j -= 1;
        }
      }
    }
    chars.iter().collect()
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
  fn test_reverse_vowels_simple() {
    let suites = vec![
      Suite {
        s: "hello",
        ret: "holle"
      },
      Suite {
        s: "leetcode",
        ret: "leotcede"
      }
    ];

    for s in suites {
      assert_eq!(s.ret.to_owned(), Solution::reverse_vowels(s.s.to_owned()));
    }
  }
}