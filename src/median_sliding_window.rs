use super::Solution;

impl Solution {
  pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
    if k == 1 || nums.is_empty() { return nums.into_iter().map(|item| item as f64).collect(); }  
    let mut window: Vec<i32> = vec![];
    let mut ret: Vec<f64> = vec![];
    for i in 0..k as usize {
      window.push(nums[i]);
    }
    window.sort_unstable();
    let mid = k as usize / 2;
    if k & 1 == 1 {
      ret.push(window[mid] as f64)
    } else {
      ret.push((window[mid] as f64 + window[mid - 1] as f64) * 0.5);
    }
    for i in k as usize .. nums.len() {
      let pop_num = nums[i - k as usize];
      let mut j = {
        Solution::binary_search(&window, pop_num, false)
      };
      window.remove(j as usize);
      j = {
        Solution::binary_search(&window, nums[i], true)
      };
      if j >= k {
        window.push(nums[i]);
      } else {
        window.insert(j as usize, nums[i]);
      }
      if k & 1 == 1 {
        ret.push(window[mid] as f64)
      } else {
        ret.push((window[mid] as f64 + window[mid - 1] as f64) * 0.5);
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
    k: i32,
    ret: Vec<f64>
  }

  #[test]
  fn median_sliding_window_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,3,-1,-3,5,3,6,7],
        k: 3,
        ret: vec![1,-1,-1,3,5,6].into_iter().map(|item| item as f64).collect()
      }
    ];

    for s in suites {
      assert_eq!(Solution::median_sliding_window(s.nums, s.k), s.ret);
    }
  }
}