use super::Solution;

impl Solution {
  pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut cache = vec![0;arr.len()];
    cache[0] = arr[0];
    let mut end = 0;
    let mut ret = vec![0;queries.len()];
    for i in 0..queries.len() {
      let q = &queries[i];
      let r = q[1] as usize;
      let l = q[0] as usize;
      if end < r {
        let mut cur = cache[end];
        while end < r {
          end += 1;
          cur ^= arr[end];
          cache[end] = cur;
        }
      }
      ret[i] = if l == 0 { cache[r] } else { cache[l - 1] ^ cache[r] }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    arr: Vec<i32>,
    queries: Vec<Vec<i32>>,
    ret: Vec<i32>
  }

  #[test]
  fn test_xor_queries_simple() {
    let suites = vec![
      Suite {
        arr: vec![1,3,4,8],
        queries: Solution::t2_i(vec![&[0,1],&[1,2],&[0,3],&[3,3]]),
        ret: vec![2,7,14,8]
      },
      Suite {
        arr: vec![4,8,2,10],
        queries: Solution::t2_i(vec![&[2,3],&[1,3],&[0,0],&[0,3]]),
        ret: vec![8,0,4,4]
      }
    ];

    for s in suites {
      assert_eq!(Solution::xor_queries(s.arr, s.queries), s.ret);
    }
  }
}