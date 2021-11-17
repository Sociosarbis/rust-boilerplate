use super::*;

impl Solution {
  pub fn max_product(words: Vec<String>) -> i32 {
    let bits_len_pair: Vec<(i32, usize)> = words
      .iter()
      .map(|item| (item.chars().fold(0, |acc, c| acc | (1 << (c as i32 - 97))), item.len()))
      .collect();
    let mut ret = 0;
    for i in 0..bits_len_pair.len() {
      let (bits, l) = bits_len_pair[i];
      for j in i + 1..bits_len_pair.len() {
        if bits_len_pair[j].0 & bits == 0 {
          let temp = bits_len_pair[j].1 * l;
          if temp > ret {
            ret = temp;
          }
        }
      }
    }
    ret as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    words: Vec<String>,
    ret: i32
  }

  #[test]
  fn test_max_product_simple() {
    let suites = vec![
      Suite {
        words: t1!["abcw","baz","foo","bar","xtfn","abcdef"],
        ret: 16
      },
      Suite {
        words: t1!["a","ab","abc","d","cd","bcd","abcd"],
        ret: 4
      },
      Suite {
        words: t1!["a","aa","aaa","aaaa"],
        ret: 0
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_product(s.words));
    }
  }
}