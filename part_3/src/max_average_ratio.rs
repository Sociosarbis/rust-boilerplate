use super::*;

use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug)]
struct Item {
  n: i32,
  t: i32,
  v: f64
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.v.eq(&other.v)
    }
}

impl Eq for Item {

}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.v.partial_cmp(&other.v)
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut h: BinaryHeap<Item> = BinaryHeap::new();
        for cls in &classes {
          h.push(Item {
            n: cls[0],
            t: cls[1],
            v: (cls[0] + 1) as f64 / (cls[1] + 1) as f64 - cls[0] as f64 / cls[1] as f64
          });
        }
        for _ in 0..extra_students {
            if let Some(mut item) = h.pop() {
              item.n += 1;
              item.t += 1;
              item.v = (item.n + 1) as f64 / (item.t + 1) as f64 - item.n as f64 / item.t as f64;
              h.push(item);
            }
        }
        let n = classes.len() as f64;
        h
            .into_iter()
            .fold(0.0, |acc, item| acc + item.n as f64 / item.t as f64)
            / n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        classes: Vec<Vec<i32>>,
        extra_students: i32,
        ret: f64,
    }

    #[test]
    fn test_max_average_ratio_simple() {
        let suites = vec![
            Suite {
                classes: t2_i![[1, 2], [3, 5], [2, 2]],
                extra_students: 2,
                ret: 0.78333,
            },
            Suite {
                classes: t2_i![[2, 4], [3, 9], [4, 5], [2, 10]],
                extra_students: 4,
                ret: 0.53485,
            },
        ];

        for s in suites {
            assert_eq!(
                (s.ret - Solution::max_average_ratio(s.classes, s.extra_students)).abs() < 1e-5,
                true
            );
        }
    }
}
