use super::*;

impl Solution {
  pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = arr.len() - 1;
    while l < r {
      let mid = (l + r) >> 1;
      if mid + 1 < arr.len() && arr[mid] <= arr[mid + 1] {
        l = mid + 1;
        continue;
      }
      if mid > 0 && arr[mid] <= arr[mid - 1] {
        r = mid - 1;
        continue;
      }
      return mid as i32;
    }
    l as i32
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
  fn test_peak_index_in_mountain_array_simple() {
    let suites = vec![
      Suite {
        arr: vec![0,1,0],
        ret: 1
      },
      Suite {
        arr: vec![1,3,5,4,2],
        ret: 2
      },
      Suite {
        arr: vec![0,10,5,2],
        ret: 1
      },
      Suite {
        arr: vec![3,4,5,1],
        ret: 2
      },
      Suite {
        arr: vec![24,69,100,99,79,78,67,36,26,19],
        ret: 2
      }
    ];
    
    for s in suites {
      assert_eq!(s.ret, Solution::peak_index_in_mountain_array(s.arr));
    }
  }
}