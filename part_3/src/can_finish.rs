use super::*;

impl Solution {
  pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut indegrees = vec![0; num_courses as usize];
    let mut dependants = vec![vec![]; num_courses as usize];
    for p in prerequisites {
      indegrees[p[0] as usize] += 1;
      dependants[p[1] as usize].push(p[0] as usize);
    }
    let mut queue = vec![];
    for (i, &indegree) in indegrees.iter().enumerate() {
      if indegree == 0 {
        queue.push(i);
      }
    }
    while !queue.is_empty() {
      let n = queue.len();
      for i in 0..n {
        let index = queue[i];
        for &d in &dependants[index] {
          indegrees[d] -= 1;
          if indegrees[d] == 0 {
            queue.push(d);
          }
        }
      }
      queue.drain(0..n);
    }
    indegrees.into_iter().find(|&d| d != 0).is_none()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    num_courses: i32,
    prerequisites: Vec<Vec<i32>>,
    ret: bool,
  }

  #[test]
  fn test_can_finish_simple() {
    let suites = vec![
      Suite {
        num_courses: 2,
        prerequisites: t2_i![[1, 0]],
        ret: true,
      },
      Suite {
        num_courses: 2,
        prerequisites: t2_i![[1, 0], [0, 1]],
        ret: false,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::can_finish(s.num_courses, s.prerequisites));
    }
  }
}
