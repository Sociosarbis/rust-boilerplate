use super::Solution;


impl Solution {
  pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
    let mut ret: Vec<Vec<i32>> = vec![];
    let chars: Vec<char> = s.chars().collect();
    let mut count = 1;
    let mut prev_char = chars[0];
    let mut prev_i = 0;
    for i in 1..chars.len() {
      if prev_char == chars[i] {
        count += 1;
      } else {
        if count > 2 {
          ret.push(vec![prev_i as i32, (i - 1) as i32])
        }
        prev_i = i;
        prev_char = chars[i];
        count = 1;
      }
    }
    if count > 2 {
      ret.push(vec![prev_i as i32, (s.len() - 1) as i32])
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    s: String,
    ret: Vec<Vec<i32>>
  }

  #[test]
  fn large_group_positions_simple() {
    let suites = vec![
      Suite {
        s: "abbxxxxzzy".to_owned(),
        ret: vec![vec![3,6]]
      },
      Suite {
        s: "abc".to_owned(),
        ret: vec![]
      },
      Suite {
        s: "abcdddeeeeaabbbcd".to_owned(),
        ret: vec![vec![3,5],vec![6,9],vec![12,14]]
      },
    ];

    for s in suites {
      assert_eq!(Solution::large_group_positions(s.s), s.ret)
    }
  }
}