use super::*;

impl Solution {
  pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut queue: Vec<usize> = (0..score.len()).collect();
    queue.sort_by(|&a, &b| score[b].cmp(&score[a]));
    let mut ret = vec![String::default();queue.len()];
    for i in 0..queue.len() {
      ret[queue[i]] = match i {
        0 => "Gold Medal".to_string(),
        1 => "Silver Medal".to_string(),
        2 => "Bronze Medal".to_string(),
        n => (n + 1).to_string()  
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    score: Vec<i32>,
    ret: Vec<String>
  }

  #[test]
  fn find_relative_ranks_simple() {
    let suites = vec![
      Suite {
        score: vec![5,4,3,2,1],
        ret: t1!["Gold Medal","Silver Medal","Bronze Medal","4","5"]
      },
      Suite {
        score: vec![10,3,8,9,4],
        ret: t1!["Gold Medal","5","Bronze Medal","Silver Medal","4"]
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_relative_ranks(s.score));
    }
  }
}