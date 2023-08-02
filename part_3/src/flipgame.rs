use super::*;

impl Solution {
  pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
    let mut map = [0u8; 10001];
    let mut ret = 0;
    for (i, num) in fronts.into_iter().enumerate() {
      let i1 = num as usize;
      let i2 = backs[i] as usize;
      if map[i1] != 2 {
        if num == backs[i] {
          map[i1] = 2;
        } else if map[i1] != 1 {
          map[i1] = 1;
        }
      }
      if map[i2] != 2 {
        if map[i2] != 1 {
          map[i2] = 1;
        }
      }
    }
    for i in 1..=1000 {
      if map[i] == 1 {
        ret = i as i32;
        break;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    fronts: Vec<i32>,
    backs: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_flipgame_simple() {
    let suites = vec![
      Suite {
        fronts: vec![1, 2, 4, 4, 7],
        backs: vec![1, 3, 4, 1, 3],
        ret: 2,
      },
      Suite {
        fronts: vec![1],
        backs: vec![1],
        ret: 0,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::flipgame(s.fronts, s.backs));
    }
  }
}
