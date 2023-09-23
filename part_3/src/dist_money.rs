use super::*;

impl Solution {
  pub fn dist_money(mut money: i32, children: i32) -> i32 {
    if money < children {
      return -1;
    }
    if money > children * 8 {
      return children - 1;
    }
    money -= children;
    let mut ret = money / 7;
    let m = money % 7;
    if ret == children {
      if m != 0 {
        ret = children - 1;
      }
    } else {
      if m == 3 {
        if ret + 1 == children {
          ret -= 1;
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
    money: i32,
    children: i32,
    ret: i32,
  }

  #[test]
  fn test_dist_money_simple() {
    let suites = vec![
      Suite {
        money: 20,
        children: 3,
        ret: 1,
      },
      Suite {
        money: 16,
        children: 2,
        ret: 2,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::dist_money(s.money, s.children));
    }
  }
}
