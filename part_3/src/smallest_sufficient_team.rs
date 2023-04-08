use super::*;

use std::collections::HashMap;

impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
      let mut m = HashMap::new();
      for (i, skill) in req_skills.iter().enumerate() {
        m.insert(skill.to_owned(), i as i32);
      }
      let mut dp: HashMap<i32, Vec<i32>> = HashMap::new();
      dp.insert(0, vec![]);
      for (i, person) in people.iter().enumerate() {
        let mut bits = 0;
        for skill in person {
          bits |= 1 << m.get(skill).unwrap();
        }
        let mut extend_dp: HashMap<i32, Vec<i32>> = HashMap::new();
        for (&k, v) in &dp {
          let next_bits = k | bits;
          let mut need_update = true;
          if let Some(ev) = dp.get(&next_bits) {
            if ev.len() <= v.len() + 1 {
              need_update = false;
            }
          }
          if let Some(ev) = extend_dp.get(&next_bits) {
            if ev.len() <= v.len() + 1 {
              need_update = false;
            }
          }
          if need_update {
            let mut next_v = v.clone();
            next_v.push(i as i32);
            extend_dp.insert(next_bits, next_v);
          }
        }
        dp.extend(extend_dp);
      }
      dp.remove(&((1 << req_skills.len()) - 1)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        req_skills: Vec<String>,
        people: Vec<Vec<String>>,
        ret: Vec<i32>,
    }

    #[test]
    fn test_smallest_sufficient_team_simple() {
        let suites = vec![
            Suite {
                req_skills: t1!["java", "nodejs", "reactjs"],
                people: t2![["java"], ["nodejs"], ["nodejs", "reactjs"]],
                ret: vec![0, 2],
            },
            Suite {
                req_skills: t1!["algorithms", "math", "java", "reactjs", "csharp", "aws"],
                people: t2![
                    ["algorithms", "math", "java"],
                    ["algorithms", "math", "reactjs"],
                    ["java", "csharp", "aws"],
                    ["reactjs", "csharp"],
                    ["csharp", "math"],
                    ["aws", "java"]
                ],
                ret: vec![1, 2],
            },
        ];

        for s in suites {
            assert_eq!(
                s.ret,
                Solution::smallest_sufficient_team(s.req_skills, s.people)
            );
        }
    }
}
