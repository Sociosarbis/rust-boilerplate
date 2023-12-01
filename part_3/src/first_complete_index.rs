use super::*;

impl Solution {
  pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut rows = vec![n as i32; m];
    let mut cols = vec![m as i32; n];
    let mut map = vec![(0, 0); m * n + 1];
    for (i, row) in mat.into_iter().enumerate() {
      for (j, cell) in row.into_iter().enumerate() {
        map[cell as usize] = (i, j);
      }
    }
    for (index, num) in arr.into_iter().enumerate() {
      let (i, j) = map[num as usize];
      rows[i] -= 1;
      cols[j] -= 1;
      if rows[i] == 0 || cols[j] == 0 {
        return index as i32;
      }
    }
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    arr: Vec<i32>,
    mat: Vec<Vec<i32>>,
    ret: i32,
  }

  #[test]
  fn test_first_complete_index_simple() {
    let suites = vec![
      Suite {
        arr: vec![1, 3, 4, 2],
        mat: t2_i![[1, 4], [2, 3]],
        ret: 2,
      },
      Suite {
        arr: vec![2, 8, 7, 4, 1, 3, 5, 6, 9],
        mat: t2_i![[3, 2, 5], [1, 4, 6], [8, 7, 9]],
        ret: 3,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::first_complete_index(s.arr, s.mat));
    }
  }
}
