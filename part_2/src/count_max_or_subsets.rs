use super::*;

fn dfs(nums: &Vec<i32>, i: usize, temp: i32, max: i32) -> i32 {
  if i < nums.len() {
    if temp == max {
      return 1 << (nums.len() - i);
    }
    let mut count = dfs(nums, i + 1, temp, max);
    let next_temp = temp | nums[i];
    if next_temp == max {
      count += 1 << (nums.len() - i - 1);
    } else {
      count += dfs(nums, i + 1, next_temp, max); 
    }
    count
  } else {
    0
  }
}

impl Solution {
  pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    dfs(&nums, 0, 0, nums.iter().fold(0, |acc, item| acc | item))
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
  fn test_count_max_or_subsets_simple() {
    let suites = vec![
      Suite {
        nums: vec![3,1],
        ret: 2
      },
      Suite {
        nums: vec![2,2,2],
        ret: 7
      },
      Suite {
        nums: vec![3,2,1,5],
        ret: 6
      }
    ];
    
    for s in suites {
      assert_eq!(s.ret, Solution::count_max_or_subsets(s.nums));
    }
  }
}