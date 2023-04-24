use super::*;

impl Solution {
  pub fn last_substring(s: String) -> String {
    let n = s.len();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut j = 1;
    let mut len = 0;
    while j + len < n {
      if chars[i + len] == chars[j + len] {
        len += 1;
      } else if chars[i + len] < chars[j + len] {
        // 这里的转化的逻辑跟下一个条件的注释一样
        i = (i + len + 1).max(j);
        j = i + 1;
        len = 0;
      } else {
        // len个字符已经比较过且相同，而[i:i + len]是目前最优，不存在以[j+1:j+len]内的字符开头的子串比它更优 
        j += len + 1;
        len = 0;
      }
    }
    s.chars().skip(i).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    s: String,
    ret: String
  }

  #[test]
  fn test_last_substring_simple() {
    let suites = vec![
      Suite {
        s: "abab".to_string(),
        ret: "bab".to_string()
      },
      Suite {
        s: "leetcode".to_string(),
        ret: "tcode".to_string()
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::last_substring(s.s));
    }
  }
}