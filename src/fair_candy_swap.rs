use super::Solution;


impl Solution {
  pub fn fair_candy_swap(mut a: Vec<i32>, mut b: Vec<i32>) -> Vec<i32> {
      let sum1 = a.iter().fold(0, |acc, item| { acc + item });
      let sum2 = b.iter().fold(0, |acc, item| { acc + item });
      let diff = sum2 - sum1;
      let mut j = 0;
      a.sort_unstable();
      b.sort_unstable();
      for i in 0..a.len() {
        let mut target = diff + (a[i] << 1);
        if (target & 1) == 0 {
          target >>= 1;
          while b[j] < target {
            j +=1;
          }
          if b[j] == target {
            return vec![a[i], b[j]];
          }
        }
      }
      vec![0, 0]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    a: Vec<i32>,
    b: Vec<i32>,
    ret: Vec<i32>
  }

  #[test]
  fn fair_candy_swap_simple() {
    let suites = vec![
      Suite {
        a: vec![1, 1],
        b: vec![2, 2],
        ret: vec![1,2]
      },
      Suite {
        a: vec![1, 2],
        b: vec![2, 3],
        ret: vec![1,2]
      },
      Suite {
        a: vec![2],
        b: vec![1,3],
        ret: vec![2,3]
      },
      Suite {
        a: vec![1,2,5],
        b: vec![2,4],
        ret: vec![5,4]
      },
    ];

    for s in suites {
      assert_eq!(Solution::fair_candy_swap(s.a, s.b), s.ret);
    }
  }
}