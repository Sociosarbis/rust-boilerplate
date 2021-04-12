use super::Solution;
use std::cmp::Ordering;

impl Solution {
  pub fn largest_number(nums: Vec<i32>) -> String {
    let mut num_digits: Vec<Vec<i32>> = nums.iter().map(|&n| {
      if n == 0 {
        return vec![0];
      }
      let mut num = n;
      let mut ret = vec![]; 
      while num != 0 {
        ret.push(num % 10);
        num /= 10;
      }
      ret
     }).collect();
     num_digits.sort_by(|a, b| {
       let m = a.len() as i32;
       let n = b.len() as i32;
       let mut i = 0;
       while i < m + n {
        let left = if m > i { a[(m - i - 1) as usize] } else { b[(m + n - i - 1) as usize] };
        let right = if n > i { b[(n - i - 1) as usize] } else { a[(m + n - i - 1) as usize]};
        if left > right {
          return Ordering::Less
        } else if left < right {
          return Ordering::Greater
        }
        i += 1;
       }
       a.len().cmp(&b.len())
     });

     if num_digits[0][num_digits[0].len() - 1] == 0 {
       return "0".to_owned()
     }
     let mut ret = String::new();
     for digits in num_digits {
       let s: String = digits.iter().rev().map(|&d| {
        std::char::from_digit(d as u32, 10).unwrap()
      }).collect();
      ret.push_str(&s);
     }
     ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: String
  }

  #[test]
  fn test_largest_number_simple() {
    let suites = vec![
      Suite {
        nums: vec![10, 2],
        ret: "210".to_owned()
      },
      Suite {
        nums: vec![3,30,34,5,9],
        ret: "9534330".to_owned(),
      },
      Suite {
        nums: vec![1],
        ret: "1".to_owned()
      },
      Suite {
        nums: vec![432,43243],
        ret: "43243432".to_owned()
      }
    ];

    for s in suites {
      assert_eq!(Solution::largest_number(s.nums), s.ret);
    }
  }
}