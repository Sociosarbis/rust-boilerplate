use super::Solution;

impl Solution {
  pub fn reverse_str(s: String, k: i32) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
      let next_i = i + 2 * k as usize;
      let mut j = i + k as usize - 1;
      if  j >= chars.len() {
        j = chars.len() - 1;
      }
      while i < j {
        chars.swap(i, j);
        i += 1;
        if j != 0 {
          j -= 1;
        }
      }
      i = next_i;
    }
    chars.iter().collect()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    s: &'a str,
    k: i32,
    ret: &'a str
  }

  #[test]
  fn test_reverse_str_simple() {
    let suites = vec![
      Suite {
        s: "abcdefg",
        k: 2,
        ret: "bacdfeg"
      },
      Suite {
        s: "abcd",
        k: 2,
        ret: "bacd"
      }
    ];

    for s in suites {
      assert_eq!(s.ret.to_owned(), Solution::reverse_str(s.s.to_owned(), s.k));
    }
  }
}