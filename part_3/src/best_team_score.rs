use super::*;

use std::cmp::Ordering;

impl Solution {
  pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
    let mut pair: Vec<(i32, i32)> = vec![];
    for i in 0..scores.len() {
      pair.push((ages[i], scores[i]));
    }
    pair.sort_by(|a, b| {
      match a.0.cmp(&b.0) {
        Ordering::Equal => {
          a.1.cmp(&b.1)
        }
        r => r 
      }
    });
    let mut dp = vec![0;pair.len()];
    dp[0] = pair[0].1;
    for i in 1..pair.len() {
      for j in 0..i {
        if pair[j].1 <= pair[i].1 && dp[i] < dp[j] {
          dp[i] = dp[j];
        }
      }
      dp[i] += pair[i].1
    }
    dp.into_iter().fold(0, |acc, item| acc.max(item))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    scores: Vec<i32>,
    ages: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_best_team_score_simple() {
    let suites = vec![
      Suite {
        scores: vec![1,3,5,10,15],
        ages: vec![1,2,3,4,5],
        ret: 34
      },
      Suite {
        scores: vec![4,5,6,5],
        ages: vec![2,1,2,1],
        ret: 16
      },
      Suite {
        scores: vec![1,2,3,5],
        ages: vec![8,9,10,1],
        ret: 6
      },
      Suite {
        scores: vec![319776,611683,835240,602298,430007,574,142444,858606,734364,896074],
        ages: vec![1,1,1,1,1,1,1,1,1,1],
        ret: 5431066
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::best_team_score(s.scores, s.ages));
    }
  }
}