use super::*;

impl Solution {
  pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let blocks: Vec<char> = blocks.chars().collect();
    let mut c = 0;
    for i in 0..k as usize {
      if blocks[i] == 'B' {
        c += 1;
      }
    }
    if c == k {
      return 0;
    }
    let mut ret = c;
    for i in k as usize..blocks.len() {
      if blocks[i - k as usize] == 'B' {
        c -= 1;
      }
      if blocks[i] == 'B' {
        c += 1;
      }
      if c > ret {
        ret = c;
      }
    }
    k - ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    blocks: String,
    k: i32,
    ret: i32
  }

  #[test]
  fn test_minimum_recolors_simple() {
    let suites = vec![
      Suite {
        blocks: "WBBWWBBWBW".to_string(),
        k: 7,
        ret: 3
      },
      Suite {
        blocks: "WBWBBBW".to_string(),
        k: 2,
        ret: 0
      },
    ];
    for s in suites {
      assert_eq!(s.ret, Solution::minimum_recolors(s.blocks, s.k));
    }
  }
}

