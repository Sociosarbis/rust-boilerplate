use super::Solution;

impl Solution {
  pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    let mut window = vec![nums[0]];
    let mut ret = 1;
    let mut j = 0;
    for i in 1..nums.len() {
      let mut index = Solution::binary_search(&window, nums[i], true) as usize;
      if index >= window.len() {
        window.push(nums[i]);
      } else {
        window.insert(index, nums[i]);
      }

      while window[window.len() - 1] - window[0] > limit {
        index = Solution::binary_search(&window, nums[j], false) as usize;
        window.remove(index);
        j +=1; 
      }
      
      if window.len() > ret {
          ret = window.len();
      }
    }
    return ret as i32;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    limit: i32,
    ret: i32
  }

  #[test]
  fn longest_subarray_simple() {
    let suites = vec![
      Suite {
        nums: vec![8,2,4,7],
        limit: 4,
        ret: 2,
      },
      Suite {
        nums: vec![10,1,2,4,7,2],
        limit: 5,
        ret: 4,
      },
      Suite {
        nums: vec![4,2,2,2,4,4,2,2],
        limit: 0,
        ret: 3,
      }
    ];

    for su in suites {
      assert_eq!(Solution::longest_subarray(su.nums, su.limit), su.ret)
    }
  }
}