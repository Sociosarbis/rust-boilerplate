use super::*;
use std::collections::HashMap;


impl Solution {
  pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
    let mut counter = HashMap::new();
    let mut ret = vec![0;barcodes.len()];
    for code in barcodes {
      if let Some(c) = counter.get_mut(&code) {
        *c += 1;
      } else {
        counter.insert(code, 1);
      }
    }
    let mut heap = Vec::with_capacity(counter.len());
    for (k, v) in counter {
      heap.push((v, k));
    }
    heap.sort_unstable();
    for i in 0..ret.len() {
      for j in (0..heap.len()).rev() {
        if i == 0 || ret[i - 1] != heap[j].1 {
          ret[i] = heap[j].1;
          let next_c = heap[j].0 - 1;
          if next_c == 0 {
            heap.drain(j..j+1);
          } else {
            heap[j].0 = next_c;
            let mut idx = j;
            while idx > 0 && heap[idx - 1].0 > next_c {
              idx -= 1;
            }
            if idx != j {
              heap.swap(idx, j);
            }
          }
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
    barcodes: Vec<i32>,
    ret: Vec<i32>
  }

  #[test]
  fn test_rearrange_barcodes_simple() {
    let suites = vec![
      Suite {
        barcodes: vec![1,1,1,2,2,2],
        ret: vec![2,1,2,1,2,1]
      },
      Suite {
        barcodes: vec![1,1,1,1,2,2,3,3],
        ret: vec![1, 3, 1, 2, 1, 2, 1, 3]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::rearrange_barcodes(s.barcodes));
    }
  }
}