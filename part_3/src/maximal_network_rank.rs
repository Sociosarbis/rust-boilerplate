use super::*;

impl Solution {
  pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut counter = vec![0;n];
    let mut m = vec![vec![false;n];n];
    for road in &roads {
      let a = road[0] as usize;
      let b = road[1] as usize;
      counter[a] += 1;
      counter[b] += 1;
      m[a][b] = true;
      m[b][a] = true;
    }
    let mut ret = 0;
    for i in 0..n {
      let a = counter[i];
      for j in i + 1..n {
        let v = a + counter[j] + if m[i][j] { -1 } else { 0 };
        if v > ret {
          ret = v;
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
    n: i32,
    roads: Vec<Vec<i32>>,
    ret: i32,
  }

  #[test]
  fn test_maximal_network_rank_simple() {
    let suites = vec![
      Suite {
        n: 4,
        roads: t2_i![[0,1],[0,3],[1,2],[1,3]],
        ret: 4,
      },
      Suite {
        n: 5,
        roads: t2_i![[0,1],[0,3],[1,2],[1,3],[2,3],[2,4]],
        ret: 5,
      },
      Suite {
        n: 8,
        roads: t2_i![[0,1],[1,2],[2,3],[2,4],[5,6],[5,7]],
        ret: 5
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::maximal_network_rank(s.n, s.roads));
    }
  }
}