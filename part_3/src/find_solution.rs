use super::*;

trait CustomFunction {
    fn f(&self, x:i32,y:i32)->i32;
 }

impl Solution {
  pub fn find_solution(customfunction: &dyn CustomFunction, z: i32) -> Vec<Vec<i32>> {
    let mut l1 = 1;
    let mut r1 = 1000;
    let mut ret = vec![];
    while l1 <= r1 {
      let mid = (l1 + r1) / 2;
      if customfunction.f(mid, 1) > z {
        r1 = mid - 1;
      } else if customfunction.f(mid + 1, 1) <= z {
        l1 = mid + 1;
      } else {
        r1 = mid;
        break;
      }
    }
    for i in 1..=r1 {
      let mut l2 = 1;
      let mut r2 = 1000;
      while l2 <= r2 {
        let mid = (l2 + r2) / 2;
        let v = customfunction.f(i, mid);
        if v > z {
          r2 = mid - 1;
        } else if v < z {
          l2 = mid + 1;
        } else {
          ret.push(vec![i, mid]);
          break;
        }
      }
    }
    ret
  }
}