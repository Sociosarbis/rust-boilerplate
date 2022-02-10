use std::ops::{ Rem, Div };
use std::fmt::{Display, Debug};

#[macro_export]
macro_rules! t2_i {
  ($($l:expr),*) => {
    Solution::t2_i(vec![$(&$l),*])
  };
}

#[macro_export]
macro_rules! t2 {
  ($($l:expr),*) => {
    Solution::t2(vec![$(&$l),*])
  };
}

#[macro_export]
macro_rules! t1 {
  ($($l:expr),*) => {
    vec![$($l.to_owned()),*]
  };
}

#[macro_export]
macro_rules! t2_c {
  ($($l:expr), *) => {
    Solution::t2_c(vec![$(&$l),*])
  }
}


pub trait Utility {

  fn t2(source: Vec<&[&str]>) -> Vec<Vec<String>> {
    source.into_iter().map(|a| {
      a.into_iter().map(|&s| { s.to_owned() }).collect()
    }).collect()
  }

  fn t2_i(source: Vec<&[i32]>) -> Vec<Vec<i32>> {
    source.into_iter().map(|s| { s.to_owned() }).collect()
  }

  fn t2_c(source: Vec<&[&str]>) -> Vec<Vec<char>> {
    source.into_iter().map(|a| { a.into_iter().map(|&s| { s.chars().next().unwrap() }).collect() }).collect()
  }

  fn binary_search<T: Ord + Eq>(nums: &Vec<T>, target: T, is_insert: bool) -> i32 {
    if nums.is_empty() {
      return if is_insert { 0 } else { -1 };
    }
    return Self::binary_search_general(nums, target, 0, nums.len() - 1, is_insert)
  }

  fn binary_search_general<T: Ord + Eq>(nums: &Vec<T>, target: T, i: usize, j: usize, is_insert: bool) -> i32 {
    let mut l = i;
    let mut r = j;
    while l <= r {
      let mid = (l + r) / 2;
      if nums[mid] < target {
        l = mid + 1;
        if l > r {
          return if is_insert { l as i32 } else { -1 }
        }
      } else if nums[mid] > target {
        if mid == 0 || mid - 1 < l {
          return if is_insert { mid as i32 } else { -1 };
        }
        r = mid - 1;
      } else {
        return mid as i32;
      }
    }
    0
  }

  fn get_greatest_common_divisor<T: Ord + Eq + Div<Output = T> + Rem<Output = T> + Into<i32> + Copy + Display + Debug>(mut a: T, mut b: T) -> T {
    loop {
      if a > b {
        std::mem::swap(&mut a, &mut b);
      }
      let next_b = b % a;
      if next_b.into() == 0 {
        break;
      }
      b = next_b;
    }
    a
  }

}