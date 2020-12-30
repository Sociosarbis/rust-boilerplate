use super::Solution;

impl Solution {
  pub fn last_stone_weight(mut stones: Vec<i32>) -> i32 {
      stones.sort_unstable();
      while stones.len() > 1 {
        let a = stones.pop().unwrap();
        let b = stones.pop().unwrap();
        if a > b {
          let c = a - b;
          let i;
          if !stones.is_empty() {
            let mut left = 0;
            let mut right = stones.len() - 1;
            loop {
              let mid = (left + right) / 2;
              if c > stones[mid] {
                left = mid + 1;
                if left > right {
                  i = left;
                  break;
                }
              } else if c < stones[mid] {
                if mid == 0 {
                  i = 0;
                  break;
                }
                right = mid - 1;
                if left > right {
                  i = mid;
                  break;
                }
              } else {
                i = mid;
                break;
              }
            }
          } else {
            i = stones.len();
          }
          if i == stones.len() { stones.push(c) } else { stones.insert(i, c) }
        }
      }
      if stones.is_empty() { 0 } else { stones[0] }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    stones: Vec<i32>,
    ret: i32
  }
  #[test]
  fn last_stone_weight_simple() {
    let suite = vec![Suite{
      stones: vec![2,7,4,1,8,1],
      ret: 1
    },
    Suite {
      stones: vec![316,157,73,106,771,828,46,212,926,604,600,992,71,51,477,869,425,405,859,924,45,187,283,590,303,66,508,982,464,398],
      ret: 0
    }];
    for s in suite {
      assert_eq!(Solution::last_stone_weight(s.stones), s.ret);
    }
  }
}