use super::*;

impl Solution {
  pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
      return s;
    }
    let mut rows: Vec<String> = vec![String::new();num_rows as usize];
    let mut i = 0;
    for c in s.chars() {
      let j = i % (num_rows * 2 - 2);
      let index = if j < num_rows {
        j
      } else {
        2 * num_rows - 2 - j
      };
      rows[index as usize].push(c);
      i += 1;
    }
    rows.into_iter().fold(String::new(), |acc, item| format!("{}{}", acc, item))
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    s: &'a str,
    num_rows: i32,
    ret: &'a str
  }

  #[test]
  fn test_convert_simple() {
    let suites = vec![
      Suite {
        s: "PAYPALISHIRING",
        num_rows: 3,
        ret: "PAHNAPLSIIGYIR"
      },
      Suite {
        s: "PAYPALISHIRING",
        num_rows: 4,
        ret: "PINALSIGYAHRPI"
      },
      Suite {
        s: "A",
        num_rows: 1,
        ret: "A"
      }
    ];

    for s in suites {
      assert_eq!(s.ret.to_string(), Solution::convert(s.s.to_string(), s.num_rows))
    }
  }
}