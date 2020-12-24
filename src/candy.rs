use super::Solution;


impl Solution {
  pub fn candy(ratings: Vec<i32>) -> i32 {
      let mut nums: Vec<i32> = vec![1;ratings.len()];
      // 正向遍历保证元素满足rating大于前面的元素时，值需要大于前面的元素
      for i in 1..ratings.len() {
        if ratings[i] > ratings[i - 1] {
            nums[i] = nums[i - 1] + 1;
        }
      }

      // 反向遍历保证元素满足rating大于后面的元素时，值需要大于后面的元素
      for i in (0..ratings.len()-1).rev() {
          if ratings[i] > ratings[i + 1] && nums[i] <= nums[i + 1] {
              nums[i] = nums[i + 1] + 1;
          }
      }
      nums.iter().sum()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn candy_simple() {
    assert_eq!(Solution::candy(vec![1,0,2]), 5);
    assert_eq!(Solution::candy(vec![1,2,2]), 4);
  }
}