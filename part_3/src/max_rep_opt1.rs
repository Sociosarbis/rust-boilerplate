use super::*;

impl Solution {
  pub fn max_rep_opt1(text: String) -> i32 {
    let mut ret = 0;
    let mut queue: Vec<(u8, i32)> = vec![(0, 0); 2];
    let mut counter: Vec<i32> = vec![0; 26];
    for b in text.bytes() {
      counter[(b - 97) as usize] += 1;
    }
    let mut bs = text.bytes();
    let mut temp: i32;
    let mut prev: u8;
    if let Some(b) = bs.next() {
      prev = b;
      temp = 1;
      for b in bs {
        if b == prev {
          temp += 1;
        } else {
          let mut next_ret = temp;
          if queue[1].1 == 1 && queue[0].0 == prev {
            next_ret += queue[0].1;
          }
          if next_ret < counter[(prev - 97) as usize] {
            next_ret += 1;
          }
          if next_ret > ret {
            ret = next_ret;
          }
          queue.push((prev, temp));
          queue.remove(0);
          temp = 1;
          prev = b;
        }
      }
      let mut next_ret = temp;
      if queue[1].1 == 1 && queue[0].0 == prev {
        next_ret += queue[0].1;
      }
      if next_ret < counter[(prev - 97) as usize] {
        next_ret += 1;
      }
      if next_ret > ret {
        ret = next_ret;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    text: String,
    ret: i32,
  }

  #[test]
  fn test_max_rep_opt1_simple() {
    let suites = vec![
      Suite {
        text: "ababa".to_string(),
        ret: 3,
      },
      Suite {
        text: "aaabaaa".to_string(),
        ret: 6,
      },
      Suite {
        text: "aaaaa".to_string(),
        ret: 5,
      },
      Suite {
        text: "aaabbaaa".to_string(),
        ret: 4,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_rep_opt1(s.text));
    }
  }
}
