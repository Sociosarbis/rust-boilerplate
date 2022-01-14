use super::*;


impl Solution {
  pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let n = if nums1.len() < k as usize { nums1.len() } else { k as usize };
    let mut options: Vec<(usize, usize)> = (0..n).rev().map(|i| (i, 0)).collect();
    let mut ret = vec![];
    while ret.len() < k as usize && !options.is_empty() {
      let item = options.pop().unwrap();
      if (item.1 + 1) < nums2.len() {
        let new_item = (item.0, item.1 + 1);
        if options.is_empty() {
          options.push(new_item);
        } else {
          let mut l = 0;
          let mut r = options.len() - 1;
          let sum = nums1[new_item.0] + nums2[new_item.1];
          while l <= r {
            let mid = (l + r) >> 1;
            let temp = nums1[options[mid].0] + nums2[options[mid].1];
            if sum < temp {
              l = mid + 1;
            } else if sum > temp {
              if mid > 0 {
                r = mid - 1;
              } else {
                break;
              }
            } else {
              l = mid;
              break;
            }
          }
          if l < options.len() {
            options.insert(l, new_item);
          } else {
            options.push(new_item);
          }
        }
      }
      ret.push(vec![nums1[item.0], nums2[item.1]]);
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    k: i32,
    ret: Vec<Vec<i32>>
  }

  #[test]
  fn test_k_smallest_pairs_simple() {
    let suites = vec![
      Suite {
        nums1: vec![1,7,11],
        nums2: vec![2,4,6],
        k: 3,
        ret: t2_i![[1,2],[1,4],[1,6]]
      },
      Suite {
        nums1: vec![1,1,2],
        nums2: vec![1,2,3],
        k: 2,
        ret: t2_i![[1,1],[1,1]]
      },
      Suite {
        nums1: vec![1,2],
        nums2: vec![3],
        k: 3,
        ret: t2_i![[1,3],[2,3]]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::k_smallest_pairs(s.nums1, s.nums2, s.k));
    }
  }
}