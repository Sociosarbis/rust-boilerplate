use super::*;

impl Solution {
  pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for i in (0..digits.len()).rev() {
      let next_num = digits[i] + 1;
      if next_num > 9 {
        digits[i] = next_num - 10;
      } else {
        digits[i] = next_num;
        break;
      }
    }
    if digits[0] == 0 {
      digits.insert(0, 1);
    }
    digits
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    digits: Vec<i32>,
    ret: Vec<i32>
  }

  #[test]
  fn test_plus_one_simple() {
    let suites = vec![
      Suite {
        digits: vec![1,2,3],
        ret: vec![1,2,4]
      },
      Suite {
        digits: vec![4,3,2,1],
        ret: vec![4,3,2,2]
      },
      Suite {
        digits: vec![0],
        ret: vec![1]
      },
      Suite {
        digits: vec![9],
        ret: vec![1,0]
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::plus_one(s.digits));
    }
  }
}