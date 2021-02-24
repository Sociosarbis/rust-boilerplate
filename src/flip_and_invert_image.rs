use super::Solution;

impl Solution {
  pub fn flip_and_invert_image(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
      let m = a.len();
      let n = a[0].len();
      for i in 0..m {
        for j in 0..(n / 2) {
          let tmp = a[i][j];
          a[i][j] = a[i][n - 1 - j];
          a[i][n - 1 - j] = tmp; 
        }
      }
      for i in 0..m {
        for j in 0..n {
          a[i][j] = 1 - a[i][j];
        }
      }
      return a;
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    a: Vec<Vec<i32>>,
    ret: Vec<Vec<i32>>
  }

  #[test]
  fn flip_and_invert_image_simple() {
    let suites = vec![
      Suite {
        a: Solution::t2_i(vec![&[1,1,0],&[1,0,1],&[0,0,0]]),
        ret: Solution::t2_i(vec![&[1,0,0],&[0,1,0],&[1,1,1]])
      },
      Suite {
        a: Solution::t2_i(vec![&[1,1,0,0],&[1,0,0,1],&[0,1,1,1],&[1,0,1,0]]),
        ret: Solution::t2_i(vec![&[1,1,0,0],&[0,1,1,0],&[0,0,0,1],&[1,0,1,0]])
      },
    ];

    for s in suites {
      assert_eq!(Solution::flip_and_invert_image(s.a), s.ret);
    }
  }
}