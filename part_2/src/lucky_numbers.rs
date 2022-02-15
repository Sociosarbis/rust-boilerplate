use super::*;

impl Solution {
  pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut max: Vec<i32> = (0..n).into_iter().map(|i| matrix[0][i]).collect();
    for i in 0..n {
      for j in 1..m {
        if matrix[j][i] > max[i] {
          max[i] = matrix[j][i];
        }
      }
    }
    let mut ret = vec![];
    for i in 0..m {
      let mut temp =  0;
      for j in 1..n {
        if matrix[i][j] < matrix[i][temp] {
          temp = j;
        }
      }
      if matrix[i][temp] == max[temp] {
        ret.push(max[temp]);
      }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    matrix: Vec<Vec<i32>>,
    ret: Vec<i32>
  }

  #[test]
  fn test_lucky_numbers_simple() {
    let suites = vec![
      Suite {
        matrix: t2_i![[3,7,8],[9,11,13],[15,16,17]],
        ret: vec![15]
      },
      Suite {
        matrix: t2_i![[1,10,4,2],[9,3,8,7],[15,16,17,12]],
        ret: vec![12]
      },
      Suite {
        matrix: t2_i![[7,8],[1,2]],
        ret: vec![7]
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::lucky_numbers(s.matrix));
    }
  }
}