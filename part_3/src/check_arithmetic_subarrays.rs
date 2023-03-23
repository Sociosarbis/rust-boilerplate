use super::*;

impl Solution {
  pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    let mut ret = vec![false;l.len()];
    'a: for i in 0..l.len() {
      let n = (r[i] - l[i] + 1) as usize;
      if n < 3 {
        ret[i] = true;
      }
      let mut temp = vec![0;n];
      for j in l[i] as usize..=r[i] as usize {
        temp[j - l[i] as usize] = nums[j];
      }
      temp.sort();
      let diff = temp[1] - temp[0];
      for j in 2..n {
        if temp[j] - temp[j - 1] != diff {
          continue 'a
        }
      }
      ret[i] = true;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    l: Vec<i32>,
    r: Vec<i32>,
    ret: Vec<bool>
  }

  #[test]
  fn test_check_arithmetic_subarrays_simple() {
    let suites = vec![
      Suite {
        nums: vec![4,6,5,9,3,7],
        l: vec![0,0,2],
        r: vec![2,3,5],
        ret: vec![true,false,true]
      },
      Suite {
        nums: vec![-12,-9,-3,-12,-6,15,20,-25,-20,-15,-10],
        l: vec![0,1,6,4,8,7],
        r: vec![4,4,9,7,9,10],
        ret: vec![false,true,false,false,true,true]
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::check_arithmetic_subarrays(s.nums, s.l, s.r));
    }
  }
}