use super::*;

impl Solution {
  pub fn minimum_deletions(s: String) -> i32 {
    let mut prefix_a = vec![0;s.len() + 1];
    for (i, c) in s.chars().enumerate() {
      if c == 'a' {
        prefix_a[i + 1] = prefix_a[i] + 1;
      } else {
        prefix_a[i + 1] = prefix_a[i];
      }
    }
    let mut ret = s.len() as i32;
    let mut temp = 0;
    for (i, c) in s.chars().enumerate() {
      if c == 'b' {
        let remain_a = prefix_a[s.len()] - prefix_a[i + 1];
        if remain_a > 0 {
          if temp + remain_a < ret {
            ret = temp + remain_a;
          }
          temp += 1;
        } else {
          break;
        }
      }
    }
    if temp < ret {
      ret = temp;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    s: String,
    ret: i32
  }

  #[test]
  fn test_minimum_deletiions_simple() {
    let suites = vec![
      Suite {
        s: "aababbab".to_string(),
        ret: 2
      },
      Suite {
        s: "bbaaaaabb".to_string(),
        ret: 2
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::minimum_deletions(s.s));
    }
  }
}

