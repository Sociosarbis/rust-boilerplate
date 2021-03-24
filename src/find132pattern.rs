use super::Solution;

impl Solution {
  pub fn find132pattern(nums: Vec<i32>) -> bool {
    if nums.len() < 3 {
      return false;
    }
    let mut stack: Vec<(usize, usize)> = vec![];
    for i in 1..nums.len() {
      let mut n = stack.len();
      for j in 0..n {
        if nums[i] > nums[stack[j].0] && nums[i] < nums[stack[j].1] {
          return true;
        }
      }

      if nums[i] > nums[i - 1] {
        if stack.is_empty() || nums[stack[n - 1].1] != nums[i - 1] {
          stack.push((i - 1, i));
          n += 1;
        } else {
          stack[n - 1].1 = i;
        }
        if n > 1 {
          let mut j = n - 1;
          while j > 0 && nums[stack[j - 1].0] >= nums[stack[n - 1].0] && nums[stack[j - 1].1] <= nums[stack[n - 1].1] {
            j -= 1;
          }
          for k in (j..n - 1).rev() {
            stack.remove(k);
          }
        }
      }
    }
    false
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  struct Suite {
    nums: Vec<i32>,
    ret: bool
  }

  #[test]
  fn find132pattern_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,2,3,4],
        ret: false
      },
      Suite {
        nums: vec![3,1,4,2],
        ret: true
      },
      Suite {
        nums: vec![-1,3,2,0],
        ret: true
      },
      Suite {
        nums: vec![3,5,0,3,4],
        ret: true,
      },
      Suite {
        nums: vec![1,2,3,3,3,4,5,3],
        ret: true
      }
    ];

    for s in suites {
      assert_eq!(Solution::find132pattern(s.nums), s.ret);
    }
  }
}