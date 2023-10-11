use super::*;

use std::{
  cmp::Ordering,
  collections::{HashMap, HashSet},
};

impl Solution {
  pub fn top_students(
    positive_feedback: Vec<String>,
    negative_feedback: Vec<String>,
    report: Vec<String>,
    mut student_id: Vec<i32>,
    k: i32,
  ) -> Vec<i32> {
    let p_set: HashSet<String> = positive_feedback.into_iter().collect();
    let n_set: HashSet<String> = negative_feedback.into_iter().collect();
    let mut points: HashMap<i32, i32> = HashMap::new();
    for (i, &id) in student_id.iter().enumerate() {
      let mut temp = String::new();
      let mut ps = 0;
      for r in report[i].chars().chain(" ".chars()) {
        if r != ' ' {
          temp.push(r);
        } else {
          if p_set.contains(&temp) {
            ps += 3;
          } else if n_set.contains(&temp) {
            ps -= 1;
          }
          temp.clear();
        }
      }
      points.insert(id, ps);
    }
    student_id.sort_unstable_by(|id_1, id_2| {
      return match points
        .get(&id_2)
        .unwrap_or(&0)
        .cmp(points.get(&id_1).unwrap_or(&0))
      {
        Ordering::Equal => id_1.cmp(id_2),
        r => r,
      };
    });
    student_id.into_iter().take(k as usize).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    positive_feedback: Vec<String>,
    negative_feedback: Vec<String>,
    report: Vec<String>,
    student_id: Vec<i32>,
    k: i32,
    ret: Vec<i32>,
  }

  #[test]
  fn test_top_students_simple() {
    let suites = vec![
      Suite {
        positive_feedback: t1!["smart", "brilliant", "studious"],
        negative_feedback: t1!["not"],
        report: t1!["this student is studious", "the student is smart"],
        student_id: vec![1, 2],
        k: 2,
        ret: vec![1, 2],
      },
      Suite {
        positive_feedback: t1!["smart", "brilliant", "studious"],
        negative_feedback: t1!["not"],
        report: t1!["this student is not studious", "the student is smart"],
        student_id: vec![1, 2],
        k: 2,
        ret: vec![2, 1],
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::top_students(
          s.positive_feedback,
          s.negative_feedback,
          s.report,
          s.student_id,
          s.k
        )
      );
    }
  }
}
