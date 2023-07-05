use super::*;

impl Solution {
  pub fn k_items_with_maximum_sum(num_ones: i32, num_zeros: i32, num_neg_ones: i32, k: i32) -> i32 {
    if k <= num_ones {
      k
    } else if k <= num_ones + num_zeros {
      num_ones
    } else {
      num_ones - (k - num_ones - num_zeros)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    num_ones: i32,
    num_zeros: i32,
    num_neg_ones: i32,
    k: i32,
    ret: i32,
  }

  #[test]
  fn test_k_items_with_maximum_sum_simple() {
    let suites = vec![
      Suite {
        num_ones: 3,
        num_zeros: 2,
        num_neg_ones: 0,
        k: 2,
        ret: 2,
      },
      Suite {
        num_ones: 3,
        num_zeros: 2,
        num_neg_ones: 0,
        k: 4,
        ret: 3,
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::k_items_with_maximum_sum(s.num_ones, s.num_zeros, s.num_neg_ones, s.k)
      );
    }
  }
}
