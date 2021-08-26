use super::*;

impl Solution {
  pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort_unstable();
    let mut i = 0;
    let mut j = people.len() - 1;
    let mut ret = 0;
    while i <= j {
      if i == j {
        i += 1;
      } else {
        if people[i] + people[j] <= limit {
          i += 1;
          j -= 1;
        } else {
          j -= 1;
        }
      }
      ret += 1;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    people: Vec<i32>,
    limit: i32,
    ret: i32
  }

  #[test]
  fn test_num_rescue_boats_simple() {
    let suites = vec![
      Suite {
        people: vec![1,2],
        limit: 3,
        ret: 1
      },
      Suite {
        people: vec![3,2,2,1],
        limit: 3,
        ret: 3
      },
      Suite {
        people: vec![3,5,3,4],
        limit: 5,
        ret: 4
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::num_rescue_boats(s.people, s.limit));
    }
  }
}