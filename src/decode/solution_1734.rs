use crate::Solution;

impl Solution {
  // 异或满足结合律： a ^ (b ^ c) = (a ^ b) ^ c
  pub fn decode_1734(encoded: Vec<i32>) -> Vec<i32> {
    let mut total = 1;
    // perm是1..n各正整数的排列，total是perm所有数的异或结果
    for i in 2..encoded.len() as i32 + 2 {
      total ^= i;
    }
    // odd是除perm[0]外所有数的异或结果
    // total ^ odd = perm[0] ^ (perm[1]^...^perm[n])^(perm[1] ^...^perm[n])
    //             = perm[0]
    let mut odd = encoded[1];
    for i in (3..encoded.len()).step_by(2) {
      odd ^= encoded[i];
    }
    Solution::decode(encoded, total ^ odd)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    encoded: Vec<i32>,
    ret: Vec<i32>
  }

  #[test]
  fn test_decode_simple() {
    let suites = vec![
      Suite {
        encoded: vec![3,1],
        ret: vec![1,2,3]
      },
      Suite {
        encoded: vec![6,5,4,6],
        ret: vec![2,4,1,5,3]
      }
    ];

    for s in suites {
      assert_eq!(Solution::decode_1734(s.encoded), s.ret);
    }
  }
}