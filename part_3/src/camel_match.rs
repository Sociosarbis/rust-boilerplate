use super::*;

impl Solution {
  pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
    let mut ret = vec![true; queries.len()];
    let pattern_chars: Vec<char> = pattern.chars().collect();
    'a: for (i, q) in queries.into_iter().enumerate() {
      let mut j = 0;
      for c in q.chars() {
        if c >= 'A' && c <= 'Z' {
          if j >= pattern_chars.len() || c != pattern_chars[j] {
            ret[i] = false;
            continue 'a;
          } else {
            j += 1;
          }
        } else {
          if j < pattern_chars.len() && c == pattern_chars[j] {
            j += 1;
          }
        }
      }
      while j < pattern_chars.len() {
        if pattern_chars[j] >= 'A' && pattern_chars[j] <= 'Z' {
          ret[i] = false;
          break;
        }
        j += 1;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    queries: Vec<String>,
    pattern: String,
    ret: Vec<bool>
  }

  #[test]
  fn test_camel_match_simple() {
    let suites = vec![
      Suite {
        queries: t1!["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"],
        pattern: "FB".to_string(),
        ret: vec![true,false,true,true,false]
      },
      Suite {
        queries: t1!["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"],
        pattern: "FB".to_string(),
        ret: vec![true,false,true,true,false]
      },
      Suite {
        queries: t1!["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"],
        pattern: "FoBa".to_string(),
        ret: vec![true,false,true,false,false]
      },
      Suite {
        queries: t1!["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"],
        pattern: "FoBaT".to_string(),
        ret: vec! [false,true,false,false,false]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::camel_match(s.queries, s.pattern));
    }
  }
}