use super::*;

impl Solution {
    pub fn min_number_of_hours(
        mut initial_energy: i32,
        mut initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
    ) -> i32 {
      let mut ret = 0;
      for &e in &energy {
        initial_energy -= e;
      }
      if initial_energy <= 0 {
        ret += -initial_energy + 1;
      }
      for &e in &experience {
        if initial_experience < e + 1 {
          ret += e + 1 - initial_experience;
          initial_experience = e + 1;
        }
        initial_experience += e;
      }
      ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        initial_energy: i32,
        initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
        ret: i32,
    }

    #[test]
    fn test_min_number_of_hours_simple() {
        let suites = vec![
            Suite {
                initial_energy: 5,
                initial_experience: 3,
                energy: vec![1, 4, 3, 2],
                experience: vec![2, 6, 3, 1],
                ret: 8,
            },
            Suite {
                initial_energy: 2,
                initial_experience: 4,
                energy: vec![1],
                experience: vec![3],
                ret: 0,
            },
        ];

        for s in suites {
            assert_eq!(
                s.ret,
                Solution::min_number_of_hours(
                    s.initial_energy,
                    s.initial_experience,
                    s.energy,
                    s.experience
                )
            );
        }
    }
}
