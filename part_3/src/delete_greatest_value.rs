use super::*;

impl Solution {
  pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
    for i in 0..grid.len() {
      grid[i].sort_unstable();
    }
    (0..grid[0].len()).fold(0, |acc, i| {
      acc + (0..grid.len()).map(|j| grid[j][i]).max().unwrap()
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    grid: Vec<Vec<i32>>,
    ret: i32,
  }

  #[test]
  fn test_delete_greatest_value_simple() {
    let suites = vec![
      Suite {
        grid: t2_i![[1, 2, 4], [3, 3, 1]],
        ret: 8,
      },
      Suite {
        grid: t2_i![[10]],
        ret: 10,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::delete_greatest_value(s.grid));
    }
  }
}
