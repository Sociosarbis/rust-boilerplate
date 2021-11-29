use super::*;

impl Solution {
  pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut queue: Vec<(usize, usize)> = (0..arr.len()).rev().map(|i| (i, arr.len() - 1)).collect();
    for _ in 0..k - 1 {
      if let Some(item) = queue.pop() {
        if item.1 > item.0 + 1 {
          let new_item = (item.0, item.1 - 1);
          let mut l = 0;
          let mut r = queue.len() - 1;
          while l <= r {
            let mid = (l + r) >> 1;
            if Solution::is_smaller(&arr, new_item, queue[mid]) {
              l = mid + 1;
            } else {
              if mid > 0 {
                r = mid - 1;
              } else {
                break;
              }
            }
          }
          if l >= queue.len() {
            queue.push(new_item);
          } else {
            queue.insert(l, new_item);
          }
        }
      }
    }
    vec![arr[queue[queue.len() - 1].0], arr[queue[queue.len() - 1].1]]
  }

  pub fn is_smaller(arr: &Vec<i32>, a: (usize, usize), b: (usize, usize)) -> bool {
    arr[a.0] * arr[b.1] < arr[a.1] * arr[b.0]
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    arr: Vec<i32>,
    k: i32,
    ret: Vec<i32>
  }

  #[test]
  fn test_kth_smallest_prime_fraction_simple() {
    let suites = vec![
      Suite {
        arr: vec![1,2,3,5],
        k: 3,
        ret: vec![2,5]
      },
      Suite {
        arr: vec![1,7],
        k: 1,
        ret: vec![1,7]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::kth_smallest_prime_fraction(s.arr, s.k));
    }
  }
}