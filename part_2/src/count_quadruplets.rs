use super::*;

impl Solution {
  pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
    let mut map = vec![vec![];101];
    let mut ret = 0;
    for i in 0..nums.len() {
      map[nums[i] as usize].push(i);
    }
    for i in 0..nums.len() - 2 {
      let a = nums[i];
      for j in i + 1..nums.len() - 1 {
        let b = nums[j];
        for k in j + 1..nums.len() {
          let target = (a + b + nums[k]) as usize;
          if target <= 100 {
            for &index in &map[target] {
              if index > k {
                ret += 1;
              }
            }
          }
        }
      }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  struct Suite {
    nums: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_count_quadruplets_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,2,3,6],
        ret: 1
      },
      Suite {
        nums: vec![3,3,6,4,5],
        ret: 0
      },
      Suite {
        nums: vec![1,1,1,3,5],
        ret: 4
      },
      Suite {
        nums: vec![9,6,8,23,39,23],
        ret: 2
      },
      Suite {
        nums: vec![28,8,49,85,37,90,20,8],
        ret: 1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_quadruplets(s.nums));
    }
  }
}