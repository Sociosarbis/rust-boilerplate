use super::*;

impl Solution {
  pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];
    let mut i = arr1.len() - 1;
    let mut j = arr2.len() - 1;
    let mut carry = 0;
    while i < arr1.len() || j < arr2.len() || carry != 0 {
      if i < arr1.len() {
        carry += arr1[i];
        if i == 0 {
          i = arr1.len();
        } else {
          i -= 1;
        }
      }
      if j < arr2.len() {
        carry += arr2[j];
        if j == 0 {
          j = arr2.len();
        } else {
          j -= 1;
        }
      }
      // carry = -1,表示当前位为1，下一位加1
      ret.push(carry & 1);
      // 当前位进1，下一位需要减1
      carry = -(carry >> 1);
    }
    while ret.len() > 1 && ret[ret.len() - 1] == 0 {
      ret.pop();
    }
    ret.reverse();
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    arr1: Vec<i32>,
    arr2: Vec<i32>,
    ret: Vec<i32>
  }

  #[test]
  fn test_add_negabinary_simple() {
    let suites = vec![
      Suite {
        arr1: vec![1,1,1,1,1],
        arr2: vec![1,0,1],
        ret: vec![1,0,0,0,0]
      },
      Suite {
        arr1: vec![0],
        arr2: vec![0],
        ret: vec![0]
      },
      Suite {
        arr1: vec![0],
        arr2: vec![1],
        ret: vec![1]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::add_negabinary(s.arr1, s.arr2));
    }
  }
}