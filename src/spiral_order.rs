use super::Solution;

impl Solution {
  pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut dir = 0;
    let mut ret = vec![];
    let mut ranges = [[0, matrix.len()], [0, matrix[0].len()]];
    let mut pos = [0, 0];
    loop {
      match dir {
        0 => {
          if ranges[1][0] >= ranges[1][1] {
            return ret;
          }
          for i in ranges[1][0]..ranges[1][1] {
            ret.push(matrix[pos[0]][i]);
          }
          ranges[0][0] += 1;
          pos[1] = ranges[1][1] - 1
        },
        1 => {
          if ranges[0][0] >= ranges[0][1] {
            return ret;
          }
          for i in ranges[0][0]..ranges[0][1] {
            ret.push(matrix[i][pos[1]]);
          }
          ranges[1][1] -= 1;
          pos[0] = ranges[0][1] - 1
        },
        2 => {
          if ranges[1][0] >= ranges[1][1] {
            return ret;
          }
          for i in (ranges[1][0]..ranges[1][1]).rev() {
            ret.push(matrix[pos[0]][i]);
          }
          ranges[0][1] -= 1;
          pos[1] = ranges[1][0]
        },
        3 => {
          if ranges[0][0] >= ranges[0][1] {
            return ret;
          }
          for i in (ranges[0][0]..ranges[0][1]).rev() {
            ret.push(matrix[i][pos[1]]);
          }
          ranges[1][0] += 1;
          pos[0] = ranges[0][0]
        },
        _ => ()
      }
      dir = (dir + 1) % 4;
    }
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
  fn spiral_order_simple() {
    let suites = vec![
      Suite {
        matrix: Solution::t2_i(vec![&[1,2,3,4],&[5,6,7,8],&[9,10,11,12]]),
        ret: vec![1,2,3,4,8,12,11,10,9,5,6,7]
      }
    ];

    for s in suites {
      assert_eq!(Solution::spiral_order(s.matrix), s.ret);
    }
  }
}