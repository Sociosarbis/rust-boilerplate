use super::*;

impl Solution {
  pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let mut ret = 0;
    let n = grid.len();
    for row in &grid {
      'a: for i in 0..n {
        for j in 0..n {
          if row[j] != grid[j][i] {
            continue 'a;
          }
        }
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
    grid: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn test_equal_pairs_simple() {
    let suites = vec![
      Suite {
        grid: t2_i![[3,2,1],[1,7,6],[2,7,7]],
        ret: 1
      },
      Suite {
        grid: t2_i![[3,1,2,2],[1,4,4,5],[2,4,2,2],[2,4,2,2]],
        ret: 3
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::equal_pairs(s.grid));
    }
  }
}