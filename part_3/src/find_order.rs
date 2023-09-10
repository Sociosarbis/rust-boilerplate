use super::*;

impl Solution {
  pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut indegrees = vec![0; num_courses as usize];
    let mut dependants = vec![vec![]; num_courses as usize];
    for p in prerequisites {
      indegrees[p[0] as usize] += 1;
      dependants[p[1] as usize].push(p[0] as usize);
    }
    let mut queue = vec![];
    for (i, &indegree) in indegrees.iter().enumerate() {
      if indegree == 0 {
        queue.push(i as i32);
      }
    }
    let mut start = 0;
    while start < queue.len() {
      let n = queue.len();
      for i in start..n {
        let index = queue[i];
        for &d in &dependants[index as usize] {
          indegrees[d as usize] -= 1;
          if indegrees[d as usize] == 0 {
            queue.push(d as i32);
          }
        }
      }
      start = n;
    }
    if indegrees.into_iter().find(|&d| d != 0).is_some() {
      Vec::with_capacity(0)
    } else {
      queue
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    num_courses: i32,
    prerequisites: Vec<Vec<i32>>,
    ret: Vec<i32>,
  }

  #[test]
  fn test_find_order_simple() {
    let suites = vec![
      Suite {
        num_courses: 2,
        prerequisites: t2_i![[1, 0]],
        ret: vec![0, 1],
      },
      Suite {
        num_courses: 4,
        prerequisites: t2_i![[1, 0], [2, 0], [3, 1], [3, 2]],
        ret: vec![0, 1, 2, 3],
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_order(s.num_courses, s.prerequisites));
    }
  }
}
