use super::*;

impl Solution {
  pub fn prev_perm_opt1(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() == 1 {
      return arr;
    }
    let mut options = vec![];
    options.push(arr.len() - 1);

    for i in (0..arr.len() - 1).rev() {
      let mut l = 0;
      let mut r = options.len() - 1;
      while l <= r {
        let mid  = (l + r) >> 1;
        if arr[i] > arr[options[mid]] {
          l = mid + 1;
        } else {
          if mid > 0 && arr[i] <= arr[options[mid - 1]] {
            r = mid - 1;
          } else {
            l = mid;
            break;
          }
        }
      }
      if l > 0 {
        arr.swap(i, options[l - 1]);
        break;
      }
      if arr[options[0]] == arr[i] {
        options[0] = i;
      } else {
        options.insert(0, i);
      }
    }
    arr
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    arr: Vec<i32>,
    ret: Vec<i32>
  }

  #[test]
  fn test_prev_perm_opt1_simple() {
    let suites = vec![
      Suite {
        arr: vec![3,2,1],
        ret: vec![3,1,2]
      },
      Suite {
        arr: vec![1,1,5],
        ret: vec![1,1,5]
      },
      Suite {
        arr: vec![1,9,4,6,7],
        ret: vec![1,7,4,6,9]
      },
      Suite {
        arr: vec![3,1,1,3],
        ret: vec![1,3,1,3]
      },
      Suite {
        arr: vec![1,9,4,4,6,7],
        ret: vec![1,7,4,4,6,9]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::prev_perm_opt1(s.arr));
    }
  }
}