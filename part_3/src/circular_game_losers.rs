use super::*;

impl Solution {
  pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
    let mut ret = vec![false; n as usize];
    ret[0] = true;
    let mut i = 0;
    let mut s = 1;
    loop {
      i = (i + s * k as usize) % (n as usize);
      if !ret[i] {
        ret[i] = true;
      } else {
        break;
      }
      s += 1;
    }
    ret
      .into_iter()
      .enumerate()
      .map(|(i, v)| (i as i32 + 1, v))
      .filter(|item| !item.1)
      .map(|item| item.0)
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    k: i32,
    ret: Vec<i32>,
  }

  #[test]
  fn test_circular_game_losers_simple() {
    let suites = vec![
      Suite {
        n: 5,
        k: 2,
        ret: vec![4, 5],
      },
      Suite {
        n: 4,
        k: 4,
        ret: vec![2, 3, 4],
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::circular_game_losers(s.n, s.k));
    }
  }
}
