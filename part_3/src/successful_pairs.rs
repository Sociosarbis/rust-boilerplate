use super::*;

impl Solution {
  pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    potions.sort_unstable();
    let mut spells: Vec<(i32, usize)> = spells
      .into_iter()
      .enumerate()
      .map(|(i, num)| (num, i))
      .collect();
    spells.sort_unstable();
    let mut r = potions.len();
    let mut ret = vec![0; spells.len()];
    for (num, i) in spells {
      while r > 0 && num as i64 * potions[r - 1] as i64 >= success {
        r -= 1;
      }
      ret[i] = (potions.len() - r) as i32;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    spells: Vec<i32>,
    potions: Vec<i32>,
    success: i64,
    ret: Vec<i32>,
  }

  #[test]
  fn test_successful_pairs_simple() {
    let suites = vec![
      Suite {
        spells: vec![5, 1, 3],
        potions: vec![1, 2, 3, 4, 5],
        success: 7,
        ret: vec![4, 0, 3],
      },
      Suite {
        spells: vec![3, 1, 2],
        potions: vec![8, 5, 8],
        success: 16,
        ret: vec![2, 0, 2],
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::successful_pairs(s.spells, s.potions, s.success)
      );
    }
  }
}
