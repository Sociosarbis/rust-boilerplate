use super::*;

impl Solution {
  pub fn simplified_fractions(n: i32) -> Vec<String> {
    let mut ret = vec![];
    for i in 1..n {
      for j in i + 1 .. n + 1{
        if i == 1 || Solution::get_greatest_common_divisor(i, j) == 1 {
          ret.push(format!("{}/{}", i, j));
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
    n: i32,
    ret: Vec<String>
  }

  #[test]
  fn test_simplified_fractions_simple() {
    let suites = vec![
      Suite {
        n: 2,
        ret: t1!["1/2"]
      },
      Suite {
        n: 3,
        ret: t1!["1/2", "1/3", "2/3"]
      },
      Suite {
        n: 4,
        ret: t1!["1/2","1/3","1/4","2/3","3/4"]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::simplified_fractions(s.n));
    }
  }
}