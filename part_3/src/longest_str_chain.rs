use super::*;

impl Solution {
  pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
    words.sort_by(|a, b| a.len().cmp(&b.len()));
    if words[0].len() == words[words.len() - 1].len() {
      return 1;
    }
    let mut index = 0;
    let mut ret = 1;
    let mut dp: Vec<i32> = vec![];
    let mut l = words[0].len();
    for (i, word) in words.iter().enumerate() {
      if word.len() == l {
        dp.push(1);
      } else {
        index = i;
        break;
      }
    }
    let is_chain = |a: &String, b: &String| -> bool {
      let mut b_chars = b.chars();
      let mut has_diff = false;
      for c1 in a.chars() {
        let mut c2 = b_chars.next().unwrap();
        if c1 != c2 {
          if has_diff {
            return false;
          }
          c2 = b_chars.next().unwrap();
          if c1 != c2 {
            return false;
          }
          has_diff = true;
        }
      }
      true
    };
    while index < words.len() {
      let mut next_dp = vec![];
      l = words[index].len();
      let prev_index = index - dp.len();
      let prev_l = words[index - 1].len();
      while index < words.len() && words[index].len() == l {
        if l == prev_l + 1 {
          let mut max = 1;
          for i in 0..dp.len() {
            if is_chain(&words[prev_index + i], &words[index]) {
              if dp[i] + 1 > max {
                max = dp[i] + 1;
              }
            }
          }
          next_dp.push(max);
          if max > ret {
            ret = max;
          }
        } else {
          next_dp.push(1);
        }
        index += 1;
      }
      dp = next_dp;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    words: Vec<String>,
    ret: i32,
  }

  #[test]
  fn test_longest_str_chain_simple() {
    let suites = vec![
      Suite {
        words: t1!["a","b","ba","bca","bda","bdca"],
        ret: 4,
      },
      Suite {
        words: t1!["xbc","pcxbcf","xb","cxbc","pcxbc"],
        ret: 5,
      },
      Suite {
        words: t1!["abcd","dbqca"],
        ret: 1
      },
      Suite {
        words: t1!["bdca","bda","ca","dca","a"],
        ret: 4
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::longest_str_chain(s.words));
    }
  }
}