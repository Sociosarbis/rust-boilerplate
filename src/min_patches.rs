use super::Solution;

impl Solution {
  pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
    let mut ret = 0;
    // 表示[0, x)间的数字可覆盖
    let mut x = 1;
    let mut i = 0;
    while x <= n {
      // 当nums[i] 在区间内或与区间相接的时候，直接增大可覆盖范围
      // 此时不需要立刻更新nums[i + 1]...的范围，因为后面也会用上nums[i]的值
      if i < nums.len() && nums[i] <= x {
        x += nums[i];
        i +=1;
      } else {
        ret += 1;
        if x < 1073741824 {
          // 相当于[0, x) + [x, x)，因为数字x刚好不在区间内，范围最大的可增加的值
          // 如果是 x - 1，增加的范围就比 x小，如果是x + 1则不能覆盖 x
          x <<= 1;
        } else {
          break;
        }
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