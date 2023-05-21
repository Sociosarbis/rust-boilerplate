use super::*;

use std::collections::BinaryHeap;

impl Solution {
  pub fn store_water(bucket: Vec<i32>, vat: Vec<i32>) -> i32 {
    let mut temp = 0;
    let mut queue = BinaryHeap::new();
    for (i, mut b) in bucket.into_iter().enumerate() {
      let v = vat[i];
      if b == 0 && v != 0 {
        b += 1;
        temp += 1;
      }
      queue.push((
        if v == 0 {
          0
        } else {
          v / b + if v % b == 0 { 0 } else { 1 }
        },
        v,
        b,
      ));
    }
    let mut ret = temp;
    if let Some(&(t, _, _)) = queue.peek() {
      ret += t;
    }
    loop {
      if let Some((mut t, v, mut b)) = queue.pop() {
        b += 1;
        t = if v == 0 {
          0
        } else {
          v / b + if v % b == 0 { 0 } else { 1 }
        };
        temp += 1;
        queue.push((t, v, b));
        if let Some(&(t, _, _)) = queue.peek() {
          if temp + t < ret {
            ret = temp + t;
          }
        }
        if temp >= ret {
          break;
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
    bucket: Vec<i32>,
    vat: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_store_water_simple() {
    let suites = vec![
      Suite {
        bucket: vec![1, 3],
        vat: vec![6, 8],
        ret: 4,
      },
      Suite {
        bucket: vec![9, 0, 1],
        vat: vec![0, 2, 2],
        ret: 3,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::store_water(s.bucket, s.vat));
    }
  }
}
