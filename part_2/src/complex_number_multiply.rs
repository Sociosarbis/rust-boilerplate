use super::*;

fn parse(num: String) -> (i32, i32) {
  let mut iter = num.split('+');
  let num1 = iter.next().unwrap().parse::<i32>().unwrap();
  let num2 = iter.next().unwrap();
  (num1, num2.get(0..num2.len() - 1).unwrap().parse::<i32>().unwrap())
}

impl Solution {
  pub fn complex_number_multiply(num1: String, num2: String) -> String {
    let num1 = parse(num1);
    let num2 = parse(num2);
    format!("{}+{}i", (num1.0 * num2.0 - num1.1 * num2.1).to_string(), (num1.0 * num2.1 + num1.1 * num2.0).to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    num1: &'a str,
    num2: &'a str,
    ret: &'a str
  }

  #[test]
  fn test_complex_number_multiply_simple() {
    let suites = vec![
      Suite {
        num1: "1+1i",
        num2: "1+1i",
        ret: "0+2i"
      },
      Suite {
        num1: "1+-1i",
        num2: "1+-1i",
        ret: "0+-2i"
      }
    ];

    for s in suites {
      assert_eq!(s.ret.to_string(), Solution::complex_number_multiply(s.num1.to_string(), s.num2.to_string()))
    }
  }
}