use super::Solution;

impl Solution {
  pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut ret = vec![first];
    // a ^ b = c => a ^ c = b or b ^ c = a
    for i in 0..encoded.len() {
      ret.push(encoded[i] ^ ret[i]);
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    encoded: Vec<i32>,
    first: i32,
    ret: Vec<i32>
  }

  #[test]
  fn test_decode_simple() {
    let suites = vec![
      Suite {
        encoded: vec![1,2,3],
        first: 1,
        ret: vec![1,0,2,1]
      },
      Suite {
        encoded: vec![6,2,7,3],
        first: 4,
        ret: vec![4,2,0,7,4]
      }
    ];

    for s in suites {
      assert_eq!(Solution::decode(s.encoded, s.first), s.ret);
    }
  }
}