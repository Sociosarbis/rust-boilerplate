use super::*;

impl Solution {
  pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
    let mut ret = 0;
    for i in 0..nums.len() {
      let mut temp = (nums[i], nums[i]);
      for j in i + 1..nums.len() {
        if nums[j] < temp.0 {
          temp.0 = nums[j];
        } else if nums[j] > temp.1 {
          temp.1 = nums[j];
        }
        ret += (temp.1 - temp.0) as i64
      }
    }
    ret
  }

  // 利用单调栈分别求出nums[i]作为最大和最小值时的次数，次数等于到上一个栈元素的距离乘到导致自己出栈的元素的距离
  pub fn sub_array_ranges_best(mut nums: Vec<i32>) -> i64 {
    let mut stack = vec![];
    let mut ret = 0;
    nums.push(-1e9 as i32 - 1);
    for i in 0..nums.len() {
      while let Some(&j) = stack.last() {
        if nums[j] > nums[i] {
          stack.pop();
          ret -= nums[j] as i64 * (i - j) as i64 * (if stack.is_empty() { j + 1 } else { j - stack.last().unwrap() }) as i64;
        } else {
          break;
        }
      }
      stack.push(i); 
    }
    nums.pop();
    stack.clear();
    nums.push(1e9 as i32 + 1);
    for i in 0..nums.len() {
      while let Some(&j) = stack.last() {
        if nums[j] < nums[i] {
          stack.pop();
          ret += nums[j] as i64 * (i - j) as i64 * (if stack.is_empty() { j + 1 } else { j - stack.last().unwrap() }) as i64;
        } else {
          break;
        }
      }
      stack.push(i); 
    }
    nums.pop();
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: i64
  }

  #[test]
  fn test_sub_array_ranges_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,2,3],
        ret: 4
      },
      Suite {
        nums: vec![1,3,3],
        ret: 4
      },
      Suite {
        nums: vec![4,-2,-3,4,1],
        ret: 59
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::sub_array_ranges_best(s.nums));
    }
  }
}