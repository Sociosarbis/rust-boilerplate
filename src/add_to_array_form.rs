use super::Solution;

impl Solution {
  pub fn add_to_array_form(a: Vec<i32>, mut k: i32) -> Vec<i32> {
      let mut k_arr: Vec<i32> = vec![];
      let mut base = 10000;
      while base != 0 {
        let res = k / base;
        if res != 0 || k_arr.len() != 0 {
          k_arr.push(res);
          k -= res * base;
        }
        base /= 10;
      }
      let mut ret = vec![];
      let mut i = 1;
      let mut res = 0;
      while i <= a.len() || i <= k_arr.len() {
        let mut sum = 0;
        if i <= a.len() {
          sum += a[a.len() - i];
        }
        if i <= k_arr.len() {
          sum += k_arr[k_arr.len() - i];
        }
        if res != 0 {
          sum += res;
        }
        res = if sum >= 10 { 
          sum -= 10;
          1 
        } else { 0 };
        ret.push(sum);
        i +=1;
      }
      if res != 0 {
        ret.push(res);
      }
      ret.reverse();
      ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    a: Vec<i32>, 
    k: i32,
    ret: Vec<i32>
  }

  #[test]
  fn add_to_array_form_simple() {
    let suites = vec![
      Suite {
        a: vec![1,2,0,0],
        k: 34,
        ret: vec![1,2,3,4]
      },
      Suite {
        a: vec![2,7,4],
        k: 181,
        ret: vec![4,5,5]
      },
      Suite {
        a: vec![2,1,5],
        k: 806,
        ret: vec![1,0,2,1]
      },
      Suite {
        a: vec![9,9,9,9,9,9,9,9,9,9],
        k: 1,
        ret: vec![1,0,0,0,0,0,0,0,0,0,0]
      }
    ];

    for s in suites {
      assert_eq!(Solution::add_to_array_form(s.a, s.k), s.ret)
    }
  }
}