use super::*;

impl Solution {
  pub fn collect_the_coins(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let n = coins.len();
    let mut neighbors = vec![vec![]; n];
    let mut neighbor_counts = vec![0; n];
    for edge in edges {
      let a = edge[0] as usize;
      let b = edge[1] as usize;
      neighbors[a].push(b);
      neighbor_counts[a] += 1;
      neighbors[b].push(a);
      neighbor_counts[b] += 1;
    }
    // 先迭代地去掉无硬币的叶子
    let mut leaves = vec![];
    for i in 0..n {
      let count = neighbor_counts[i];
      if count == 1 && coins[i] == 0 {
        neighbor_counts[i] = 0;
        leaves.push(i);
      }
    }
    while !leaves.is_empty() {
      let c = leaves.len();
      for index in 0..c {
        let i = leaves[index];
        for &j in &neighbors[i] {
          if neighbor_counts[j] > 0 {
            neighbor_counts[j] -= 1;
            if neighbor_counts[j] == 1 && coins[j] == 0 {
              neighbor_counts[j] = 0;
              leaves.push(j);
            }
          }
        }
      }
      leaves.drain(0..c);
    }
    // 再把有硬币叶子及它的父节点去掉
    for i in 0..n {
      let count = neighbor_counts[i];
      if count == 1 {
        neighbor_counts[i] = 0;
        leaves.push(i);
      }
    }
    for i in leaves {
      for &j in &neighbors[i] {
        if neighbor_counts[j] > 0 {
          neighbor_counts[j] -= 1;
          if neighbor_counts[j] == 1 {
            neighbor_counts[j] = 0;
          }
        }
      }
    }
    // 剩下节点任意选为起始点，走过的边数为(n - 1) * 2
    (neighbor_counts
      .into_iter()
      .fold(0, |acc, count| acc + if count > 0 { 1 } else { 0 })
      - 1)
      .max(0)
      * 2
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    coins: Vec<i32>,
    edges: Vec<Vec<i32>>,
    ret: i32,
  }

  #[test]
  fn test_collect_the_coins_simple() {
    let suites = vec![
      Suite {
        coins: vec![1, 0, 0, 0, 0, 1],
        edges: t2_i![[0, 1], [1, 2], [2, 3], [3, 4], [4, 5]],
        ret: 2,
      },
      Suite {
        coins: vec![0, 0, 0, 1, 1, 0, 0, 1],
        edges: t2_i![[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [5, 6], [5, 7]],
        ret: 2,
      },
      Suite {
        coins: vec![1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1],
        edges: t2_i![
          [0, 1],
          [1, 2],
          [2, 3],
          [1, 4],
          [4, 5],
          [5, 6],
          [6, 7],
          [3, 8],
          [6, 9],
          [7, 10],
          [10, 11],
          [10, 12],
          [7, 13],
          [12, 14],
          [13, 15],
          [14, 16],
          [15, 17],
          [10, 18]
        ],
        ret: 12,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::collect_the_coins(s.coins, s.edges));
    }
  }
}
