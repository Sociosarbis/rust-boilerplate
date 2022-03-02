use super::*;

impl Solution {
  // 列出回文的各种可能，然后取最小的那个
  pub fn nearest_palindromic(n: String) -> String {
    let mut options = [0;4];
    let l = n.len();
    options[0] = 10_i64.pow(l as u32 - 1) - 1;
    let num = (n.chars().take(l / 2 + (l & 1)).collect::<String>()).parse::<i64>().unwrap();
    let n = n.parse::<i64>().unwrap();
    for i in -1..=1 {
      let mut num = num + i;
      let prefix = num.to_string();
      let suffix: String = prefix.chars().rev().skip(l & 1).collect();
      num = format!("{}{}", prefix, suffix).parse::<i64>().unwrap();
      options[(2 + i) as usize] = num;
    }
    let mut ret = 10_i64.pow(l as u32) + 1;
    let mut min = (ret - n).abs();
    for option in options {
      let temp = (option - n).abs();
      if (temp < min || temp == min && ret > option) && temp != 0 {
        min = temp;
        ret = option;
      }
    }
    ret.to_string()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    n: &'a str,
    ret: &'a str
  }

  #[test]
  fn test_nearest_palindromic_simple() {
    let suites = vec![
      Suite {
        n: "1234",
        ret: "1221"
      },
      Suite {
        n: "123",
        ret: "121"
      },
      Suite {
        n: "1",
        ret: "0"
      }
    ];

    for s in suites {
      assert_eq!(s.ret.to_string(), Solution::nearest_palindromic(s.n.to_string()));
    }
  }
}