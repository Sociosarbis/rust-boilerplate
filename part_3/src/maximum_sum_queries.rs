use super::*;

impl Solution {
  pub fn maximum_sum_queries(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut options: Vec<(i32, i32)> = nums1
      .into_iter()
      .enumerate()
      .map(|(i, num)| (num, nums2[i]))
      .collect();
    options.sort_unstable();
    let mut ret = vec![-1; queries.len()];
    let mut queries: Vec<(i32, i32, usize)> = queries
      .into_iter()
      .enumerate()
      .map(|(i, item)| (item[0], item[1], i))
      .collect();
    queries.sort_unstable();
    // y升序，x + y降序，淘汰不符合同时这两个规则的成员
    let mut stack: Vec<(i32, i32)> = vec![];
    for (x_min, y_min, i) in queries.into_iter().rev() {
      while let Some((x, y)) = options.pop() {
        if x < x_min {
          options.push((x, y));
          break;
        } else {
          let value = x + y;
          if stack.is_empty() {
            stack.push((y, value));
          } else {
            let mut l = 0;
            let mut r = stack.len() - 1;
            while l <= r {
              let mid = (l + r) >> 1;
              if y > stack[mid].0 {
                l = mid + 1;
              } else {
                if mid > 0 {
                  r = mid - 1;
                } else {
                  l = mid;
                  break;
                }
              }
            }
            if l < stack.len() {
              if value < stack[l].1 {
                continue;
              }
            } else {
              stack.push((y, value));
            }
            r = l;
            while l > 0 && stack[l - 1].1 <= value {
              l -= 1;
            }
            stack.drain(l..r);
          }
        }
      }
      if !stack.is_empty() {
        let mut l = 0;
        let mut r = stack.len() - 1;
        while l <= r {
          let mid = (l + r) >> 1;
          if y_min > stack[mid].0 {
            l = mid + 1;
          } else {
            if mid > 0 && y_min <= stack[mid - 1].0 {
              r = mid - 1;
            } else {
              l = mid;
              break;
            }
          }
        }
        if l < stack.len() {
          ret[i] = stack[l].1;
        }
      }
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
    queries: Vec<Vec<i32>>,
    ret: Vec<i32>,
  }

  #[test]
  fn test_maximum_sum_queries_simple() {
    let suites = vec![
      Suite {
        nums1: vec![4, 3, 1, 2],
        nums2: vec![2, 4, 9, 5],
        queries: t2_i![[4, 1], [1, 3], [2, 5]],
        ret: vec![6, 10, 7],
      },
      Suite {
        nums1: vec![3, 2, 5],
        nums2: vec![2, 3, 4],
        queries: t2_i![[4, 4], [3, 2], [1, 1]],
        ret: vec![9, 9, 9],
      },
      Suite {
        nums1: vec![2, 1],
        nums2: vec![2, 3],
        queries: t2_i![[3, 3]],
        ret: vec![-1],
      },
      Suite {
        nums1: vec![72,44],
        nums2: vec![60,86],
        queries: t2_i![[33,14]],
        ret: vec![132]
      }
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::maximum_sum_queries(s.nums1, s.nums2, s.queries)
      );
    }
  }
}
