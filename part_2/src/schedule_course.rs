use super::*;

impl Solution {
  pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {

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