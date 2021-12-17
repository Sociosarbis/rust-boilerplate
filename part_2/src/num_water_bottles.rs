use super::*;

impl Solution {
  pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
    let mut ret = 0;
    while num_bottles >= num_exchange {
      let count = num_bottles / num_exchange;
      ret += count * num_exchange;
      num_bottles = count + num_bottles % num_exchange;
    }
    ret + num_bottles
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    num_bottles: i32,
    num_exchange: i32,
    ret: i32
  }

  #[test]
  fn test_num_water_bottles_simple() {
    let suites = vec![
      Suite {
        num_bottles: 9,
        num_exchange: 3,
        ret: 13
      },
      Suite {
        num_bottles: 15,
        num_exchange: 4,
        ret: 19
      },
      Suite {
        num_bottles: 5,
        num_exchange: 5,
        ret: 6
      },
      Suite {
        num_bottles: 2,
        num_exchange: 3,
        ret: 2
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::num_water_bottles(s.num_bottles, s.num_exchange));
    }
  }
}