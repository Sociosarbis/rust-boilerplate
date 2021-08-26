use super::*;

impl Solution {
  pub fn int_to_roman(mut num: i32) -> String {
    let unit = vec![1, 4, 5, 9, 10, 40, 50, 90, 100, 400, 500, 900, 1000];
    let unit_str = vec!["I", "IV", "V", "IX", "X", "XL", "L", "XC", "C", "CD", "D", "CM", "M"];
    let mut ret = String::new();
    while num != 0 {
      let mut i = Solution::binary_search(&unit, num, true) as usize;
      if i == unit.len() {
        i = unit.len() - 1;
      } else if num < unit[i] {
        i -= 1;
      }
      num -= unit[i];
      ret.push_str(unit_str[i]);
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    num: i32,
    ret: &'a str
  }

  #[test]
  fn test_int_to_roman_simple() {
    let suites = vec![
      Suite {
        num: 3,
        ret: "III"
      },
      Suite {
        num: 4,
        ret: "IV"
      },
      Suite {
        num: 9,
        ret: "IX"
      },
      Suite {
        num: 58,
        ret: "LVIII"
      },
      Suite {
        num: 1994,
        ret: "MCMXCIV"
      }
    ];

    for s in suites {
      assert_eq!(Solution::int_to_roman(s.num), s.ret.to_owned());
    }
  }
}