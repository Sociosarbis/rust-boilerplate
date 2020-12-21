use super::Solution;

impl Solution {
  pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
      let mut min_cost: [i32;2] = [0;2];
      min_cost[0] = cost[0];
      min_cost[1] = cost[1];
      for i in 2..cost.len() {
        let tmp = if min_cost[0] < min_cost[1] { cost[i] + min_cost[0] } else { cost[i] + min_cost[1] };
        if (i & 1) == 1 { min_cost[1] = tmp; } else { min_cost[0] = tmp; }
      }
      if min_cost[0] < min_cost[1] { min_cost[0] } else { min_cost[1] }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn min_cost_climbing_stairs_simple() {
    let suites: [(Vec<i32>, i32);2] = [(vec![10, 15, 20], 15), (vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1], 6)];
    for s in suites.iter() {
      assert_eq!(Solution::min_cost_climbing_stairs(s.0.to_owned()), s.1);
    }
  }
}