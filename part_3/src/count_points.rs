use super::*;

impl Solution {
  pub fn count_points(rings: String) -> i32 {
    let mut ret = [0; 10];
    let mut chars = rings.chars();
    while let Some(c) = chars.next() {
      let i = chars.next().unwrap().to_digit(10).unwrap() as usize;
      match c {
        'R' => {
          ret[i] |= 1;
        }
        'G' => {
          ret[i] |= 2;
        }
        _ => {
          ret[i] |= 4;
        }
      }
    }
    ret.iter().filter(|&&num| num == 7).count() as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    rings: String,
    ret: i32,
  }

  #[test]
  fn test_count_points_simple() {
    let suites = vec![
      Suite {
        rings: "B0B6G0R6R0R6G9".to_string(),
        ret: 1,
      },
      Suite {
        rings: "B0R0G0R9R0B0G0".to_string(),
        ret: 1,
      },
      Suite {
        rings: "G4".to_string(),
        ret: 0,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_points(s.rings));
    }
  }
}
