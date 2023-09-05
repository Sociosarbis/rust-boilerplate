use super::*;

impl Solution {
  pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut counter: [u8; 10] = [0; 10];
    for &num in &nums1 {
      counter[num as usize] = 1;
    }
    for &num in &nums2 {
      counter[num as usize] += 1;
    }
    for num in 1..10 {
      if counter[num as usize] == 2 {
        return num;
      }
    }
    let a = nums1.into_iter().min().unwrap();
    let b = nums2.into_iter().min().unwrap();
    if a < b {
      a * 10 + b
    } else {
      b * 10 + a
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_min_number_simple() {
    let suites = vec![
      Suite {
        nums1: vec![4, 1, 3],
        nums2: vec![5, 7],
        ret: 15,
      },
      Suite {
        nums1: vec![3, 5, 2, 6],
        nums2: vec![3, 1, 7],
        ret: 3,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_number(s.nums1, s.nums2));
    }
  }
}
