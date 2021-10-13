use super::*;

static FIZZ: &str = "Fizz";
static BUZZ: &str = "Buzz";
static FIZZ_BUZZ: &str = "FizzBuzz";
impl Solution {
  pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..n + 1).map(|i| if i % 15 == 0 {
      FIZZ_BUZZ.to_owned()
    } else if i % 3 == 0 {
      FIZZ.to_owned()
    } else if i % 5 == 0 {
      BUZZ.to_owned()
    } else {
      i.to_string()
    }).collect()
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
  fn test_fizz_buzz_simple() {
    let suites = vec![
      Suite {
        n: 3,
        ret: t1!["1","2","Fizz"]
      },
      Suite {
        n: 5,
        ret: t1!["1","2","Fizz","4","Buzz"]
      },
      Suite {
        n: 15,
        ret: t1!["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::fizz_buzz(s.n));
    }
  }
}