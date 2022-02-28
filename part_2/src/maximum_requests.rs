use super::*;

fn dfs(requests: &Vec<Vec<i32>>, i: usize, n: usize, counter: &mut Vec<i32>) -> bool {
  if n == 0 {
    for item in counter {
      if *item != 0 {
        return false;
      }
    }
    return true;
  }
  for j in i..requests.len() - n + 1 {
    counter[requests[j][0] as usize] -= 1;
    counter[requests[j][1] as usize] += 1;
    if dfs(requests, j + 1, n - 1, counter) {
      return true;
    }
    counter[requests[j][0] as usize] += 1;
    counter[requests[j][1] as usize] -= 1;
  }
  false
}


impl Solution {
  pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
    let mut counter = vec![0;n as usize];
    for i in (1..=requests.len()).rev() {
      if dfs(&requests, 0, i, &mut counter) {
        return i as i32;
      }
    }
    0
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    requests: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn test_maximum_requests_simple() {
    let suites = vec![
      Suite {
        n: 5,
        requests: t2_i![[0,1],[1,0],[0,1],[1,2],[2,0],[3,4]],
        ret: 5,
      },
      Suite {
        n: 3,
        requests: t2_i![[0,0],[1,2],[2,1]],
        ret: 3,
      },
      Suite {
        n: 4,
        requests: t2_i![[0,3],[3,1],[1,2],[2,0]],
        ret: 4 
      },
      Suite {
        n: 3,
        requests: t2_i![[1,2],[1,2],[2,2],[0,2],[2,1],[1,1],[1,2]],
        ret: 4
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::maximum_requests(s.n, s.requests))
    }
  }
}