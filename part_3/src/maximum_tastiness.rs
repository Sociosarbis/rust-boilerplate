use super::*;

impl Solution {
  pub fn maximum_tastiness(mut price: Vec<i32>, k: i32) -> i32 {
    price.sort();
    let mut r = price[price.len() - 1] - price[0];
    if r == 0 {
      return 0;
    }
    let mut l = 0;
    let mut ret = 0;
    while l <= r {
      let mid = (l + r) >> 1;
      let mut c = 1;
      let mut temp = price[0];
      let mut ll = 1;
      let mut rr = price.len() - 1;
      while ll <= rr {
        let mm = (ll + rr) >> 1;
        if price[mm] - temp >= mid {
          if mm > 0 && price[mm - 1] - temp >= mid {
            rr = mm - 1;
          } else {
            ll = mm + 1;
            temp = price[mm];
            c += 1;
            if c == k {
              break;
            }
          }
        } else {
          ll = mm + 1;
        }
      }
      for &p in &price {
        if p - temp >= mid {
          c += 1;
          temp = p;
          if c == k {
            break;
          }
        }
      }
      if c < k {
        r = mid - 1;
      } else {
        l = mid + 1;
        ret = mid;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    price: Vec<i32>,
    k: i32,
    ret: i32
  }

  #[test]
  fn test_maximum_tastiness_simple() {
    let suites = vec![
      Suite {
        price: vec![13,5,1,8,21,2],
        k: 3,
        ret: 8,
      },
      Suite {
        price: vec![1,3,1],
        k: 2,
        ret: 2
      },
      Suite {
        price: vec![7,7,7,7],
        k: 2,
        ret: 0
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::maximum_tastiness(s.price, s.k));
    }
  }
}