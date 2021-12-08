use super::*;

impl Solution {
  pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut dp: Vec<Vec<Option<(i32, i32)>>> = vec![vec![None;3];nums.len()];
    let mut temp = (0..k).into_iter().fold(0, |acc, i| acc + nums[i as usize]);
    dp[0][0] = Some((temp,0));
    for i in 1..nums.len() - k as usize + 1 {
      temp += nums[i + k as usize - 1] - nums[i - 1];
      dp[i][0] = if temp > dp[i - 1][0].unwrap().0 { Some((temp, i as i32)) } else { dp[i - 1][0] };
      if i >= k as usize {
        for j in 1..3 {
          if let Some(item) = dp[i - k as usize][j - 1] {
            let v = item.0 + temp;
            if let Some(prev) = dp[i - 1][j] {
              if v > prev.0 {
                dp[i][j] = Some((v, i as i32));
              } else {
                dp[i][j] = Some(prev);
              }
            } else {
              dp[i][j] = Some((v, i as i32));
            }
          }
        }
      }
    }
    let i1 = dp[nums.len() - k as usize][2].unwrap().1;
    let i2 = dp[(i1 - k) as usize][1].unwrap().1;
    let i3 = dp[(i2 - k) as usize][0].unwrap().1;
    vec![i3, i2, i1]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    k: i32,
    ret: Vec<i32>
  }

  #[test]
  fn test_max_sum_of_three_subarrays_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,2,1,2,6,7,5,1],
        k: 2,
        ret: vec![0,3,5]
      },
      Suite {
        nums: vec![1,2,1,2,1,2,1,2,1],
        k: 2,
        ret: vec![0,2,4]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_sum_of_three_subarrays(s.nums, s.k));
    }
  }
}