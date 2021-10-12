use super::*;

impl Solution {
  pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
    let left_negative = dividend < 0;
    let right_negative = divisor < 0;
    if left_negative && dividend != -2147483648 {
      dividend = -dividend;
    }
    if right_negative && divisor != -2147483648 {
      divisor = -divisor;
    }
    let mut u_dividend = dividend as u32;
    let u_divisor = divisor as u32;
    if u_dividend < u_divisor {
      return 0;
    }
    let mut count1 = 0;
    let mut count2 = 0;
    let mut temp = u_dividend;
    while temp != 0 {
      count1 += 1;
      temp >>= 1;
    }
    temp = u_divisor;
    while temp != 0 {
      count2 += 1;
      temp >>= 1;
    }
    let mut ret = 0;
    let mut i = count1 - count2;
    temp = u_divisor << i;
    while i >= 0 && u_dividend >= u_divisor {
      if u_dividend >= temp {
        ret |= 1 << i;
        u_dividend -= temp;
      }
      temp >>= 1;
      i -= 1;
    }
    if left_negative != right_negative { 
      -ret 
    } else { 
      if ret == -2147483648 { 2147483647 } else { ret } 
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    dividend: i32,
    divisor: i32,
    ret: i32
  }

  #[test]
  fn test_divide_simple() {
    let suites = vec![
      Suite {
        dividend: 10,
        divisor: 3,
        ret: 3
      },
      Suite {
        dividend: 7,
        divisor: -3,
        ret: -2
      },
      Suite {
        dividend: 0,
        divisor: 1,
        ret: 0
      },
      Suite {
        dividend: 1,
        divisor: 1,
        ret: 1
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::divide(s.dividend, s.divisor));
    }
  }
}