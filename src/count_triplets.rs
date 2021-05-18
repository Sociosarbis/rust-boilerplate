use super::Solution;

impl Solution {
  pub fn count_triplets(arr: Vec<i32>) -> i32 {
    let mut ret = 0;
    for i in 0..arr.len() {
      let mut res = arr[i];
      for j in i + 1..arr.len() {
        res ^= arr[j];
        if res == 0 {
          ret += j - i
        } 
      }
    }
    ret as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    arr: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_count_triplets_simple() {
    let suites = vec![
      Suite {
        arr: vec![2,3,1,6,7],
        ret: 4
      },
      Suite {
        arr: vec![1,1,1,1,1],
        ret: 10
      },
      Suite {
        arr: vec![2,3],
        ret: 0
      },
      Suite {
        arr: vec![1,3,5,7,9],
        ret: 3
      },
      Suite {
        arr: vec![7,11,12,9,5,2,7,17,22],
        ret: 8
      }
    ];

    for s in suites {
      assert_eq!(Solution::count_triplets(s.arr), s.ret);
    }
  }
}