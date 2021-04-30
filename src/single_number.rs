use super::Solution;

impl Solution {
  pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut a = 0;
    let mut b = 0;
    // 以a的第n位和b的第n位组成一个二进制数
    // 表示num的二进制第n位累加后除3的余的结果
    // 由于按位计算是可以各位同时进行，所以不需要遍历各位进行转换
    // 每次遍历nums的位运算转换是可以看作是以下一位逻辑运算的简化和推广形式
    // x ^ 0 = x x ^ 1 = !x
    // b = if !a == 1 { if num == 0 { b } else { 1 } } else { 0 }
    // a = if !b == 1 { if num == 0 { a } else { 1 } } else { 0 }
    // 因为结果求的是没有重复的那个数（其他数都重复了三次），因此该数的二进制必然为余为1的结果，而这正好是b的值
    for num in nums {
      b = b ^ num & !a;
      a = a ^ num & !b; 
    }
    b
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
  fn test_single_number_simple() {
    let suites = vec![
      Suite {
        nums: vec![2,2,3,2],
        ret: 3
      },
      Suite {
        nums: vec![0,1,0,1,0,1,99],
        ret: 99
      }
    ];

    for s in suites {
      assert_eq!(Solution::single_number(s.nums), s.ret);
    }
  }
}