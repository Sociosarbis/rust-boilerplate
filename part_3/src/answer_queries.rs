use super::*;

impl Solution {
  pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    nums.sort();
    for i in 1..nums.len() {
      nums[i] += nums[i - 1];
    }
    let mut ret = vec![0;queries.len()];
    for (i, query) in queries.into_iter().enumerate() {
      let mut l = 0;
      let mut r = nums.len() - 1;
      while l <= r {
        let mid = (l + r) >> 1;
        if nums[mid] <= query {
          l = mid + 1;
        } else {
          if mid > 0 && nums[mid - 1] > query {
            r = mid - 1;
          } else {
            l = mid;
            break;
          }
        }
      }
      ret[i] = l as i32;
    }
    ret 
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    queries: Vec<i32>,
    ret: Vec<i32>
  }

  #[test]
  fn test_answer_queries_simple() {
    let suites = vec![
      Suite {
        nums: vec![4,5,2,1],
        queries: vec![3,10,21],
        ret: vec![2,3,4]
      },
      Suite {
        nums: vec![2,3,4,5],
        queries: vec![1],
        ret: vec![0]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::answer_queries(s.nums, s.queries));
    }
  }
}