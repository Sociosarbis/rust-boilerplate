use super::*;

impl Solution {
  pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
    let mut c1 = [0;4];
    let mut c2 = [0;13];
    for (i, &r) in ranks.iter().enumerate() {
      let i1 = suits[i] as usize - 97;
      let i2 = r as usize - 1;
      c1[i1] += 1;
      c2[i2] += 1;
    }
    let mut max = c1.iter().fold(0, |acc, &v| { if v >  acc { v } else { acc }});
    if max == 5 {
      return "Flush".to_string();
    }
    max = c2.iter().fold(0, |acc, &v| { if v >  acc { v } else { acc }});
    if max >= 3 {
      return "Three of a Kind".to_string();
    }
    if max == 2 {
      return "Pair".to_string();
    }
    "High Card".to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    ranks: Vec<i32>,
    suits: Vec<char>,
    ret: String
  }

  #[test]
  fn test_best_hand_simple() {
    let suites = vec![
      Suite {
        ranks: vec![13,2,3,1,9],
        suits: vec!['a','a','a','a','a'],
        ret: "Flush".to_string()
      },
      Suite {
        ranks: vec![4,4,2,4,4],
        suits: vec!['d','a','a','b','c'],
        ret: "Three of a Kind".to_string()
      },
      Suite {
        ranks: vec![10,10,2,12,9],
        suits: vec!['a','b','c','a','d'],
        ret: "Pair".to_string()
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::best_hand(s.ranks, s.suits));
    }
  }
}