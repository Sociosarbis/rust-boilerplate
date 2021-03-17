use super::Solution;

impl Solution {
  pub fn num_distinct(s: String, t: String) -> i32 {
    if s.len() < t.len() {
      return 0;
    }
    let mut dp = vec![vec![0;s.len()];2];
    let mut pool = vec![];
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    for i in 0..s_chars.len() - t_chars.len() + 1 {
      if s_chars[i] == t_chars[0] {
        pool.push(i);
        dp[0][i] = 1;
      }
    }
    for i in 1..t_chars.len() {
      let index = i % 2;
      let mut next_pool = vec![];
      let r = s_chars.len() - t_chars.len() + i + 1;
      if pool.is_empty() {
        break
      }
      let mut j = 0;
      let mut acc = 0;
      for k in pool[0] + 1..r {
        if s_chars[k] == t_chars[i] {
          next_pool.push(k);
          while j < pool.len() && k > pool[j] {
            acc += dp[1 - index][pool[j]];
            j += 1;
          }
          dp[index][k] += acc;
        }
      }
      for k in 0..pool.len() {
        dp[1 - index][pool[k]] = 0;
      }
      pool = next_pool;
    }
    let index = (t_chars.len() - 1) % 2;
    dp[index][t_chars.len() - 1..].iter().fold(0, |acc, num| { acc + num })
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    s: String,
    t: String,
    ret: i32
  }

  #[test]
  fn num_distinct_simple() {
    let suites = vec![
      Suite {
        s: "rabbbit".to_string(),
        t: "rabbit".to_string(),
        ret: 3
      },
      Suite {
        s: "babgbag".to_string(),
        t: "bag".to_string(),
        ret: 5
      },
      Suite {
        s: "adbdadeecadeadeccaeaabdabdbcdabddddabcaaadbabaaedeeddeaeebcdeabcaaaeeaeeabcddcebddebeebedaecccbdcbcedbdaeaedcdebeecdaaedaacadbdccabddaddacdddc".to_string(),
        t: "bcddceeeebecbc".to_string(),
        ret: 700531452
      }
    ];

    for s in suites {
      assert_eq!(Solution::num_distinct(s.s, s.t), s.ret);
    }
  }
}