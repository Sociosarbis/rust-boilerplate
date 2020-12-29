use super::Solution;

impl Solution {
  pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
    let filtered_nums = nums.into_iter().filter(|item| { *item <= n }).collect::<Vec<i32>>();
    let mut max_val: i32 = filtered_nums.iter().sum();
    if max_val > n {
      max_val = n;
    }
    let mut is_exists = vec![false; max_val as usize];
    Solution::get_combinations(&filtered_nums, &mut is_exists, 0, 0);
    let mut i = 0;
    let mut ret = 0;
    while i < n as usize && i < is_exists.len() as usize {
      if !is_exists[i] {
        ret += 1;
        let tmp_is_exists = is_exists.to_owned();
        let left = if i > 0 { i - 1 } else { 0 }; 
        for j in (left..tmp_is_exists.len()).rev() {
          if tmp_is_exists[j] {
            let k = j + i + 1;
            if k < n as usize {
              if is_exists.len() <= k {
                is_exists.resize(k + 1, false);
              }
              if !is_exists[k] {
                is_exists[k] = true;
              }
            }
          }
        }
        is_exists[i] = true;
      }
      i += i + 1;
    }
    if i > 0 {
      i -= 1;
    }
    while i < n as usize {
      ret += 1;
      i += i + 1;
      if n as usize > i && n as usize - i < i + 1 {
        ret += 1;
        break;
      }
    }
    ret
  }

  fn get_combinations(nums:&Vec<i32>, combinations: &mut Vec<bool>, i: usize, prev: i32) {
    for j in i..nums.len() {
      let num = nums[j] + prev;
      if num <= combinations.len() as i32 && !combinations[num as usize - 1] {
        combinations[num as usize - 1] = true;
      } else {
        continue
      }
      Solution::get_combinations(nums, combinations, j + 1, num);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    n: i32,
    ret: i32
  }

  #[test]
  fn min_patches_simple() {
    let suites: Vec<Suite> = vec![
      Suite {
        nums: vec![1, 3],
        n: 6,
        ret: 1,
      },
      Suite {
        nums: vec![1,5,10],
        n: 20,
        ret: 2
      },
      Suite {
        nums: vec![1,2,2],
        n: 5,
        ret: 0
      },
      Suite {
        nums: vec![1,2,31,33],
        n: 2147483647,
        ret: 28
      },
      Suite {
        nums: vec![],
        n: 7,
        ret: 3
      },
      Suite {
        nums: vec![8,12,21,30,33,33,41,48,49,49,50,51,56,57,61,62,67,72,73,74,85,89,90,91,92,94,97,98,99],
        n: 70,
        ret: 3
      },
      Suite {
        nums: vec![1,2,2,6,34,38,41,44,47,47,56,59,62,73,77,83,87,89,94],
        n: 20,
        ret: 1
      },
      Suite {
        nums: vec![2,3,3,4,6,8,8,10,10,10,12,13,13,14,15,15,16,17,19,20,20,21,21,21,23,23,24,25,26,27,27,28,28,29,29,30,30,31,31,32,32,32,36,36,38,41,41,41,43,44,46,46,46,48,48,49,50,51,51,52,52,53,54,55,56,56,58,58,58,59,60,60,60,61,63,63,66,66,70,70,73,74,74,75,78,80,81,83,85,87,87,89,89,89,90,90,92,92,96,98],
        n: 60844,
        ret: 5
      },
      Suite {
        nums: vec![1,2,32],
        n: 2147483647,
        ret: 28
      }
    ];

    for s in suites {
      assert_eq!(Solution::min_patches(s.nums, s.n), s.ret);
    }
  }
}