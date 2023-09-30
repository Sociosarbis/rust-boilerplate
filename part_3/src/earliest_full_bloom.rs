use super::*;

impl Solution {
  pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
    let mut pairs: Vec<(i32, i32)> = plant_time
      .into_iter()
      .enumerate()
      .map(|(i, t)| (grow_time[i], t))
      .collect();
    pairs.sort_unstable();
    pairs
      .into_iter()
      .rev()
      .fold((0, 0), |acc, (gt, pt)| {
        (acc.0.max(acc.1 + gt + pt), acc.1 + pt)
      })
      .0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    plant_time: Vec<i32>,
    grow_time: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_earliest_full_bloom_simple() {
    let suites = vec![
      Suite {
        plant_time: vec![1, 4, 3],
        grow_time: vec![2, 3, 1],
        ret: 9,
      },
      Suite {
        plant_time: vec![1, 2, 3, 2],
        grow_time: vec![2, 1, 2, 1],
        ret: 9,
      },
      Suite {
        plant_time: vec![1],
        grow_time: vec![1],
        ret: 2,
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::earliest_full_bloom(s.plant_time, s.grow_time)
      );
    }
  }
}
