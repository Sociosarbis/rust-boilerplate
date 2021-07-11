use super::Solution;

impl Solution {
  pub fn h_index(mut citations: Vec<i32>) -> i32 {
    citations.sort();
    let mut i = 0;
    let mut ret = 0;
    while i < citations.len() {
      let num = citations[i];
      while ret + 1 <= num && ret + 1 <= (citations.len() - i) as i32 {
        ret += 1;
      }
      while i + 1 < citations.len() && citations[i] == citations[i + 1] {
        i += 1;
      }
      i += 1;
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    citations: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_h_index_simple() {
    let suites = vec![
      Suite {
        citations: vec![3,0,6,1,5],
        ret: 3
      },
      Suite {
        citations: vec![1,3,1],
        ret: 1
      },
      Suite {
        citations: vec![100],
        ret: 1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::h_index(s.citations));
    }
  }
}