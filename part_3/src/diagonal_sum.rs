use super::*;

impl Solution {
  pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let mut ret = 0;
    let mut j = 0;
    let n = mat.len();
    for row in &mat {
      ret += row[j];
      j += 1;
    }
    j = 0;
    if n % 2 == 1 {
      ret -= mat[n / 2][n / 2];
    }
    for row in mat.into_iter().rev() {
      ret += row[j];
      j += 1;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    mat: Vec<Vec<i32>>,
    ret: i32,
  }

  #[test]
  fn test_diagonal_sum_simple() {
    let suites = vec![
      Suite {
        mat: t2_i![[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        ret: 25,
      },
      Suite {
        mat: t2_i![[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]],
        ret: 8,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::diagonal_sum(s.mat));
    }
  }
}
