use super::Solution;

impl Solution {
  pub fn convert_to_title(mut column_number: i32) -> String {
    let mut chars = vec![];
    while column_number != 0 {
      let mut residual = column_number % 26;
      column_number = column_number / 26;
      if residual == 0 {
        residual += 26;
        column_number -= 1;
      }
      chars.push((residual + 64) as u8 as char)
    }
    chars[0..chars.len()].iter().rev().collect()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    column_number: i32,
    ret: &'a str
  }

  #[test]
  fn test_convert_to_title_simple() {
    let suites = vec![
      Suite {
        column_number: 1,
        ret: "A"
      },
      Suite {
        column_number: 28,
        ret: "AB"
      },
      Suite {
        column_number: 701,
        ret: "ZY"
      },
      Suite {
        column_number: 2147483647,
        ret: "FXSHRXW"
      },
    ];

    for s in suites {
      assert_eq!(s.ret.to_owned(), Solution::convert_to_title(s.column_number));
    }
  }
}