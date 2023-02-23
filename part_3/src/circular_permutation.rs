use super::*;

impl Solution {
  fn binary_to_gray(num: i32) -> i32 {
    num ^ (num >> 1)
  }

  fn gray_to_binary(mut num: i32) -> i32 {
    num ^= num >> 16;
    num ^= num >> 8;
    num ^= num >> 4;
    num ^= num >> 2;
    num ^= num >> 1;
    num
  }

  // 格雷码的二进制表示相邻相差一个bit，所以先求出格雷码为start是第几个
  pub fn circular_permutation(mut n: i32, start: i32) -> Vec<i32> {
    n = 1 << n;
    let mut ret = vec![0;n as usize];
    ret[0] = start;
    let mut i = Solution::gray_to_binary(start);
    for j in 1..ret.len() {
      i += 1;
      i %= n;
      ret[j] = Solution::binary_to_gray(i);
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    start: i32,
    ret: Vec<i32>
  }

  #[test]
  fn test_circular_permutation_simple() {
    let suites = vec![
      // Suite {
      //   n: 2,
      //   start: 3,
      //   ret: vec![3,2,0,1],
      // },
      Suite {
        n: 3,
        start: 2,
        ret: vec![2,6,7,5,4,0,1,3],
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::circular_permutation(s.n, s.start));
    }
  }
}