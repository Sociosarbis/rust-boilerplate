use super::*;

impl Solution {
  pub fn find_max_k(nums: Vec<i32>) -> i32 {
    let mut ret = -1;
    let mut visited = [false;2001];
    for num in nums {
      let index = (num + 1000) as usize;
      if !visited[index] {
        visited[index] = true;
      }
      let neg_index = (-num + 1000) as usize;
      if num < 0 {
        if visited[neg_index] && -num > ret {
          ret = -num;
        }
      } else if visited[neg_index] && num > ret {
        ret = num;
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
  fn test_find_max_k_simple() {
    let suites = vec![
      Suite {
        nums: vec![-1,2,-3,3],
        ret: 3
      },
      Suite {
        nums: vec![-1,10,6,7,-7,1],
        ret: 7
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_max_k(s.nums));
    }
  }
}