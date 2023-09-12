use super::*;

use std::collections::HashSet;

impl Solution {
  pub fn check_if_prerequisite(
    num_courses: i32,
    prerequisites: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
  ) -> Vec<bool> {
    let mut indegrees = vec![0; num_courses as usize];
    let mut dependants = vec![vec![]; num_courses as usize];
    for p in prerequisites {
      indegrees[p[1] as usize] += 1;
      dependants[p[0] as usize].push(p[1]);
    }
    let mut queue = vec![];
    for (i, &indegree) in indegrees.iter().enumerate() {
      if indegree == 0 {
        queue.push(i as i32);
      }
    }
    let mut ps = vec![HashSet::new(); num_courses as usize];
    while !queue.is_empty() {
      let n = queue.len();
      for i in 0..n {
        for &num in &dependants[queue[i] as usize] {
          indegrees[num as usize] -= 1;
          ps[num as usize].insert(queue[i]);
          let iter = ps[queue[i] as usize].clone();
          ps[num as usize].extend(iter);
          if indegrees[num as usize] == 0 {
            queue.push(num);
          }
        }
      }
      queue.drain(0..n);
    }
    queries
      .into_iter()
      .map(|q| ps[q[1] as usize].contains(&q[0]))
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    num_courses: i32,
    prerequisites: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
    ret: Vec<bool>,
  }

  #[test]
  fn test_check_if_prerequisite_simple() {
    let suites = vec![
      Suite {
        num_courses: 2,
        prerequisites: t2_i![[1, 0]],
        queries: t2_i![[0, 1], [1, 0]],
        ret: vec![false, true],
      },
      Suite {
        num_courses: 2,
        prerequisites: t2_i![],
        queries: t2_i![[1, 0], [0, 1]],
        ret: vec![false, false],
      },
      Suite {
        num_courses: 3,
        prerequisites: t2_i![[1, 2], [1, 0], [2, 0]],
        queries: t2_i![[1, 0], [1, 2]],
        ret: vec![true, true],
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::check_if_prerequisite(s.num_courses, s.prerequisites, s.queries)
      )
    }
  }
}
