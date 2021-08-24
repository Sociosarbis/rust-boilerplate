use super::*;
use std::collections::HashMap;

impl Solution {
  pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let mut route_to_price: Vec<HashMap<i32, i32>> = vec![HashMap::new();n as usize];
    for f in flights {
      route_to_price[f[0] as usize].insert(f[1], f[2]);
    }
    let mut dp = vec![vec![-1;k as usize + 1];n as usize];
    dp[src as usize][0] = 0;
    let mut queue = vec![(src, 0, 0)];
    let mut ret = -1;
    while !queue.is_empty() {
      let mut new_queue = vec![];
      for item in queue {
        for route in &route_to_price[item.0 as usize] {
          let d = *route.0 as usize;
          let cost = item.1 + route.1;
          if dp[d][item.2] == -1 || dp[d][item.2] > cost {
            dp[d][item.2] = cost;
            if *route.0 == dst && (ret == -1 || cost < ret) {
              ret = cost;
            }
            if item.2 != k as usize {
              new_queue.push((*route.0, cost, item.2 + 1));
            }
          }
        }
      }
      queue = new_queue;
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    flights: Vec<Vec<i32>>,
    src: i32,
    dst: i32,
    k: i32,
    ret: i32
  }

  #[test]
  fn test_find_cheapest_price_simple() {
    let suites = vec![
      Suite {
        n: 3,
        flights: t2_i![[0,1,100],[1,2,100],[0,2,500]],
        src: 0,
        dst: 2,
        k: 1,
        ret: 200
      },
      Suite {
        n: 3,
        flights: t2_i![[0,1,100],[1,2,100],[0,2,500]],
        src: 0,
        dst: 2,
        k: 0,
        ret: 500
      },
      Suite {
        n: 4,
        flights: t2_i![[0,1,1],[0,2,5],[1,2,1],[2,3,1]],
        src: 0,
        dst: 3,
        k: 1,
        ret: 6 
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_cheapest_price(s.n, s.flights, s.src, s.dst, s.k));
    }
  }
}