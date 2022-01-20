use super::*;

fn dfs(counter: &mut [i32], i: usize, count: usize) -> i32 {
  if counter[0] + counter[1] + counter[2] == 0 {
    return 2
  }
  for j in 0..3 {
    if counter[j] > 0 {
      if (j + count) % 3 != 0 {
        counter[j] -= 1;
        let ret = dfs(counter, i + 1, (count + j) % 3);
        counter[j] += 1;
        if ret == 1 << (i & 1) {
          return ret
        }
      }
    }
  }
  1 << (1 - (i & 1))
}


impl Solution {
  pub fn stone_game_ix(stones: Vec<i32>) -> bool {
    let mut counter = [0;3];
    for &s in &stones {
      counter[s as usize % 3] += 1;
    }
    counter[0] &= 1;
    if counter[1] > 1 {
      counter[1] -= 2;
      let num = if counter[1] < counter[2] { counter[1] } else { counter[2] };
      counter[1] -= num;
      counter[2] -= num;
      if dfs(&mut counter, 0, 2) == 1 {
        return true
      }
      counter[1] += num + 2;
      counter[2] += num;
    }
    if counter[2] > 1 {
      counter[2] -= 2;
      let num = if counter[1] < counter[2] { counter[1] } else { counter[2] };
      counter[1] -= num;
      counter[2] -= num;
      if dfs(&mut counter, 0, 1) == 1 {
        return true
      }
      counter[1] += num;
      counter[2] += num + 2;
    }
    if (counter[1] <= 1 || counter[2] <= 1) && dfs(&mut counter, 0, 0) == 1 {
      return true
    }
    false
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    stones: Vec<i32>,
    ret: bool
  }


  #[test]
  fn test_stone_game_ix_simple() {
    let suites = vec![
      Suite {
        stones: vec![2,1],
        ret: true
      },
      Suite {
        stones: vec![2],
        ret: false
      },
      Suite {
        stones: vec![5,1,2,4,3],
        ret: false
      },
      Suite {
        stones: vec![7,10,1,9,19,17,1,9,19],
        ret: true
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::stone_game_ix(s.stones));
    }
  }
}