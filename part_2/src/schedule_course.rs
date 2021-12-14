use super::*;

use std::collections::BinaryHeap;

impl Solution {
  pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
    let mut t = 0;
    let mut q: BinaryHeap<i32> = BinaryHeap::new();
    courses.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
    for c in courses {
      t += c[0];
      q.push(c[0]);
      if t > c[1] {
        t -= q.pop().unwrap();
      }
    }
    q.len() as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    courses: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn test_schedule_course_simple() {
    let suites = vec![
      Suite {
        courses: t2_i![[100,200],[200,1300],[1000,1250],[2000,3200]],
        ret: 3
      },
      Suite {
        courses: t2_i![[1,2]],
        ret: 1
      },
      Suite {
        courses: t2_i![[3,2],[4,3]],
        ret: 0
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::schedule_course(s.courses));
    }
  }
}