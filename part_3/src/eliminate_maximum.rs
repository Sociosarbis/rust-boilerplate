use super::*;

impl Solution {
  pub fn eliminate_maximum(mut dist: Vec<i32>, speed: Vec<i32>) -> i32 {
    for i in 0..dist.len() {
      dist[i] = dist[i] / speed[i] + if dist[i] % speed[i] == 0 { 0 } else { 1 };
    }
    dist.sort_unstable();
    let mut ret = 0;
    for (c, t) in dist.into_iter().enumerate() {
      if t <= c as i32 {
        break;
      } else {
        ret += 1;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    dist: Vec<i32>,
    speed: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_eliminate_maximum_simple() {
    let suites = vec![
      Suite {
        dist: vec![1, 3, 4],
        speed: vec![1, 1, 1],
        ret: 3,
      },
      Suite {
        dist: vec![1, 1, 2, 3],
        speed: vec![1, 1, 1, 1],
        ret: 1,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::eliminate_maximum(s.dist, s.speed));
    }
  }
}
