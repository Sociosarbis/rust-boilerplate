use super::*;

impl Solution {
  pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
    let mut max = 0;
    let mut id = 0;
    let mut prev = 0;
    for log in logs {
      let temp = log[1] - prev;
      if temp > max {
        max = temp;
        if log[0] != id {
          id = log[0];
        }
      } else if temp == max && log[0] < id {
        id = log[0];
      }
      prev = log[1];
    }
    id
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    logs: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn test_hardest_worker_simple() {
    let suites = vec![
      Suite {
        n: 10,
        logs: t2_i![[0,3],[2,5],[0,9],[1,15]],
        ret: 1
      },
      Suite {
        n: 26,
        logs: t2_i![[1,1],[3,7],[2,12],[7,17]],
        ret: 3
      },
      Suite {
        n: 2,
        logs: t2_i![[0,10],[1,20]],
        ret: 0
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::hardest_worker(s.n, s.logs));
    }
  }
}