use super::*;

impl Solution {
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ret = vec![0; n as usize - 1];
        let mut conn = vec![vec![]; n as usize];
        for e in &edges {
            let a = e[0] as usize - 1;
            let b = e[1] as usize - 1;
            conn[a].push(b);
            conn[b].push(a);
        }
        Solution::count_subgraphs_for_each_diameter_dfs(n, 0, 0, 0, &conn, &mut ret);
        ret
    }

    pub fn count_subgraphs_for_each_diameter_dfs(
        n: i32,
        i: i32,
        mask: i32,
        count: i32,
        conn: &Vec<Vec<usize>>,
        ret: &mut Vec<i32>,
    ) {
      if i >= n {
        return;
      }
      let next_mask = mask | (1 << i);
      let next_count = count + 1;
      Solution::count_subgraphs_for_each_diameter_dfs(n, i + 1, mask, count, conn, ret);
      Solution::count_subgraphs_for_each_diameter_dfs(n, i + 1, next_mask, next_count, conn, ret);
      if next_count > 1 {
        let mut max = 0;
        for j in 0..n {
          if (next_mask & (1 << j)) != 0 {
            let mut count = 0;
            let mut visited = 1 << j;
            let mut bfs = vec![j];
            while !bfs.is_empty() {
              let n = bfs.len();
              count += 1;
              for k in 0..n {
                for &next in &conn[bfs[k] as usize] {
                  if (next_mask & (1 << next as i32)) != 0 && (visited & (1 << next as i32)) == 0 {
                    visited |= 1 << next;
                    bfs.push(next as i32);
                  }
                }
              }
              bfs.drain(0..n);
            }
            if next_mask == visited as i32 {
              if max < count {
                max = count;
              }
            } else {
              return;
            }
          }
        }
        ret[max as usize - 2] += 1;
        return;
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        n: i32,
        edges: Vec<Vec<i32>>,
        ret: Vec<i32>,
    }

    #[test]
    fn test_count_subgraphs_for_each_diameter_simple() {
        let suites = vec![
            // Suite {
            //     n: 4,
            //     edges: t2_i![[1, 2], [2, 3], [2, 4]],
            //     ret: vec![3, 4, 0],
            // },
            // Suite {
            //     n: 2,
            //     edges: t2_i![[1, 2]],
            //     ret: vec![1],
            // },
            // Suite {
            //     n: 3,
            //     edges: t2_i![[1, 2], [2, 3]],
            //     ret: vec![2, 1],
            // },
            Suite {
              n: 4,
              edges: t2_i![[1,3],[1,4],[2,3]],
              ret: vec![3,2,1]
            }
        ];

        for s in suites {
            assert_eq!(
                s.ret,
                Solution::count_subgraphs_for_each_diameter(s.n, s.edges)
            )
        }
    }
}
