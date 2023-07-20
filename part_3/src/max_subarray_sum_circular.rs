use super::*;

impl Solution {
  pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let mut prefix_sums: Vec<(i32, usize)> = vec![(0, 0)];
    let n = nums.len();
    let mut temp = 0;
    let mut ret = -30000;
    for (i, num) in nums.iter().enumerate() {
      temp += num;
      if temp - prefix_sums[0].0 > ret {
        ret = temp - prefix_sums[0].0;
      }
      while let Some(v) = prefix_sums.pop() {
        if v.0 < temp {
          prefix_sums.push(v);
          break;
        }
      }
      prefix_sums.push((temp, i + 1));
    }
    let mut index = 0;
    for (i, num) in nums.into_iter().enumerate().take(n - 1) {
      while index < prefix_sums.len() && prefix_sums[index].1 <= i {
        index += 1;
      }
      temp += num;
      if temp - prefix_sums[index].0 > ret {
        ret = temp - prefix_sums[index].0;
      }
      while let Some(v) = prefix_sums.pop() {
        if v.0 < temp || prefix_sums.len() < index {
          prefix_sums.push(v);
          break;
        }
      }
      prefix_sums.push((temp, i + n + 1));
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_subarray_sum_circular_simple() {
    let suites = vec![
      Suite {
        nums: vec![1, -2, 3, -2],
        ret: 3,
      },
      Suite {
        nums: vec![5, -3, 5],
        ret: 10,
      },
      Suite {
        nums: vec![-3, -2, -3],
        ret: -2,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_subarray_sum_circular(s.nums));
    }
  }
}
