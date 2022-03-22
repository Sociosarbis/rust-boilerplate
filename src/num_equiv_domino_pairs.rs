use super::*;

impl Solution {
  pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    let mut counters: [[i32;10];10] = [[0;10];10];
    for d in dominoes {
      let mut l = d[0];
      let mut r = d[1];
      if r < l {
        let tmp = r;
        r = l;
        l = tmp;
      }
      counters[l as usize][r as usize] += 1;
    }
    let mut ret = 0;
    for r in &counters {
      for &c in r {
        if c > 1 {
          ret += c * (c - 1) / 2;
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
    dominoes: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn num_equiv_domino_pairs_simple() {
    let suites = vec![
      Suite {
        dominoes: t2_i![[1,2],[2,1],[3,4],[5,6]],
        ret: 1
      }
    ];

    for s in suites {
      assert_eq!(Solution::num_equiv_domino_pairs(s.dominoes), s.ret);
    }
  }
}