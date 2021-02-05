use super::Solution;

impl Solution {
  pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let n = s.len();
    let s1: Vec<char> = s.chars().collect();
    let s2: Vec<char> = t.chars().collect();
    let mut arr: Vec<i32> = vec![];
    let mut ret = 0;
    let mut tmp = 0;
    let mut j = 0;
    for i in 0..n {
      let cost = (s1[i] as i32 - s2[i] as i32).abs();
      if cost <= max_cost {
        while cost + tmp > max_cost && j < arr.len() {
          tmp -= arr[j];
          j += 1;
        }
        tmp += cost;
        arr.push(cost);
        if (arr.len() - j) as i32 > ret {
          ret = (arr.len() - j) as i32;
        } 
      } else {
        j = arr.len();
        tmp = 0;
      }   
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    s: String,
    t: String,
    max_cost: i32,
    ret: i32
  }

  #[test]
  fn equal_substring_simple() {
    let suites = vec![
      Suite {
        s: "abcd".to_owned(),
        t: "bcdf".to_owned(),
        max_cost: 3,
        ret: 3 
      },
      Suite {
        s: "abcd".to_owned(),
        t: "cdef".to_owned(),
        max_cost: 3,
        ret: 1
      },
      Suite {
        s: "abcd".to_owned(),
        t: "acde".to_owned(),
        max_cost: 0,
        ret: 1
      },
      Suite {
        s: "krpgjbjjznpzdfy".to_owned(),
        t: "nxargkbydxmsgby".to_owned(),
        max_cost: 14,
        ret: 4
      }
    ];

    for s in suites {
      assert_eq!(Solution::equal_substring(s.s, s.t, s.max_cost), s.ret);
    }
  }
}