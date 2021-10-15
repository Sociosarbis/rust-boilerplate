use super::*;

impl Solution {
  pub fn count_and_say(n: i32) -> String {
    let mut ret: Vec<i32> = vec![1];
    for _ in 2..n + 1 {
      let mut new_ret = vec![];
      let mut num = ret[0];
      let mut count = 0;
      for &item in &ret {
        if item == num {
          count += 1;
        } else {
          new_ret.push(count);
          new_ret.push(num);
          num = item;
          count = 1;
        }
      }
      new_ret.push(count);
      new_ret.push(num);
      ret = new_ret;
    }
    ret.into_iter().map(|item| item.to_string()).collect()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    n: i32,
    ret: &'a str
  }

  #[test]
  fn test_count_and_say_simple() {
    let suites = vec![
      Suite {
        n: 1,
        ret: "1"
      },
      Suite {
        n: 4,
        ret: "1211"
      },
    ];

    for s in suites {
      assert_eq!(s.ret.to_owned(), Solution::count_and_say(s.n));
    }
  }
}