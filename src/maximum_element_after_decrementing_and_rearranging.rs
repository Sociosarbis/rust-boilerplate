use super::Solution;


impl Solution {
  pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
    arr.sort();
    arr[0] = 1;
    for i in 1..arr.len() {
      if arr[i] > arr[i - 1] + 1 {
        arr[i] = arr[i - 1] + 1;
      }
    }
    arr[arr.len() - 1]
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
  fn test_maximum_element_after_decrementing_and_rearranging_simple() {
    let suites = vec![
      Suite {
        arr: vec![2,2,1,2,1],
        ret: 2,
      },
      Suite {
        arr: vec![100,1,1000],
        ret: 3
      },
      Suite {
        arr: vec![1,2,3,4,5],
        ret: 5
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::maximum_element_after_decrementing_and_rearranging(s.arr));
    }
  }
}