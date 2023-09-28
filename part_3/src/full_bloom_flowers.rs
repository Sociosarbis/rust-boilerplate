use super::*;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
  pub fn full_bloom_flowers(mut flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![0; people.len()];
    flowers.sort_unstable();
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut people: Vec<(i32, usize)> = people
      .into_iter()
      .enumerate()
      .map(|(i, t)| (t, i))
      .collect();
    people.sort_unstable();
    let mut j = 0;
    for (t, i) in people {
      while j < flowers.len() {
        if flowers[j][0] <= t {
          heap.push(Reverse(flowers[j][1]));
          j += 1;
        } else {
          break;
        }
      }
      while let Some(Reverse(end)) = heap.pop() {
        if end >= t {
          heap.push(Reverse(end));
          break;
        }
      }
      ret[i] = heap.len() as i32;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    flowers: Vec<Vec<i32>>,
    people: Vec<i32>,
    ret: Vec<i32>,
  }

  #[test]
  fn test_full_bloom_flowers_simple() {
    let suites = vec![
      Suite {
        flowers: t2_i![[1, 6], [3, 7], [9, 12], [4, 13]],
        people: vec![2, 3, 7, 11],
        ret: vec![1, 2, 2, 2],
      },
      Suite {
        flowers: t2_i![[1, 10], [3, 3]],
        people: vec![3, 3, 2],
        ret: vec![2, 2, 1],
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::full_bloom_flowers(s.flowers, s.people));
    }
  }
}
