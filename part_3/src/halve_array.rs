use super::*;

use std::{collections::BinaryHeap, ops::Sub};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Fraction(i64, i64);

impl Fraction {
  fn halve(&mut self) {
    self.1 *= 2;
    self.simplify();
  }

  fn simplify(&mut self) {
    let mut a = self.0.abs();
    let mut b = self.1.abs();
    if a != 0 && a < b {
      std::mem::swap(&mut a, &mut b);
    }
    while a % b != 0 {
      let temp = a % b;
      a = b;
      b = temp;
    }
    self.0 /= b;
    self.1 /= b;
  }
}

impl Sub for Fraction {
  type Output = Fraction;

  fn sub(self, rhs: Self) -> Self::Output {
    let mut out = Fraction(self.0 * rhs.1 - rhs.0 * self.1, self.1 * rhs.1);
    out.simplify();
    out
  }
}

impl PartialOrd for Fraction {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Fraction {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    (self.0 * other.1).cmp(&(other.0 * self.1))
  }
}

impl Solution {
  pub fn halve_array(nums: Vec<i32>) -> i32 {
    let mut heap: BinaryHeap<Fraction> = BinaryHeap::new();
    let mut temp = Fraction(0, 1);
    let mut ret = 0;
    for num in nums {
      temp.0 += num as i64;
      heap.push(Fraction(num as i64, 1));
    }
    temp.halve();
    while temp.0 > 0 {
      if let Some(mut item) = heap.pop() {
        item.halve();
        temp = temp - item;
        heap.push(item);
        ret += 1;
      }
    }
    ret
  }
}

#[cfg(test)]
mod test {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_halve_array_simple() {
    let suites = vec![
      Suite {
        nums: vec![5, 19, 8, 1],
        ret: 3,
      },
      Suite {
        nums: vec![3, 8, 20],
        ret: 3,
      },
      Suite {
        nums: vec![6, 58, 10, 84, 35, 8, 22, 64, 1, 78, 86, 71, 77],
        ret: 9,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::halve_array(s.nums));
    }
  }
}
