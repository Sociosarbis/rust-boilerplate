use super::*;

impl Solution {
  pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut m = vec![false;58];
    for j in jewels.bytes() {
      m[(j - b'A') as usize] = true;
    }
    let mut ret = 0;
    for s in stones.bytes() {
      if s >= b'A' && s <= b'z' && m[(s - b'A') as usize] {
        ret += 1;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    jewels: String,
    stones: String,
    ret: i32,
  }

  #[test]
  fn test_num_jewels_in_stones_simple() {
    let suites = vec![
      Suite {
        jewels: "aA".to_string(),
        stones: "aAAbbbb".to_string(),
        ret: 3,
      },
      Suite {
        jewels: "z".to_string(),
        stones: "ZZ".to_string(),
        ret: 0,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::num_jewels_in_stones(s.jewels, s.stones));
    }
  }
}
