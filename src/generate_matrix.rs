use super::*;

impl Solution {
  pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![vec![0;n as usize];n as usize];
    let mut dir = 0;
    let mut ranges = [[0, n as usize], [0, n as usize]];
    let mut pos = [0, 0];
    let mut num = 1;
    loop {
      match dir {
        0 => {
          if ranges[1][0] >= ranges[1][1] {
            return ret;
          }
          for i in ranges[1][0]..ranges[1][1] {
            ret[pos[0]][i] = num;
            num += 1;
          }
          ranges[0][0] += 1;
          pos[1] = ranges[1][1] - 1
        },
        1 => {
          if ranges[0][0] >= ranges[0][1] {
            return ret;
          }
          for i in ranges[0][0]..ranges[0][1] {
            ret[i][pos[1]] = num;
            num += 1;
          }
          ranges[1][1] -= 1;
          pos[0] = ranges[0][1] - 1
        },
        2 => {
          if ranges[1][0] >= ranges[1][1] {
            return ret;
          }
          for i in (ranges[1][0]..ranges[1][1]).rev() {
            ret[pos[0]][i] = num;
            num += 1;
          }
          ranges[0][1] -= 1;
          pos[1] = ranges[1][0]
        },
        3 => {
          if ranges[0][0] >= ranges[0][1] {
            return ret;
          }
          for i in (ranges[0][0]..ranges[0][1]).rev() {
            ret[i][pos[1]] = num;
            num += 1;
          }
          ranges[1][0] += 1;
          pos[0] = ranges[0][0];
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
    n: i32,
    ret: Vec<Vec<i32>>
  }

  #[test]
  fn generate_matrix_simple() {
    let suites = vec![
      Suite {
        n: 3,
        ret: t2_i![[1,2,3],[8,9,4],[7,6,5]]
      },
      Suite {
        n: 1,
        ret: t2_i![[1]]
      }
    ];

    for s in suites {
      assert_eq!(Solution::generate_matrix(s.n), s.ret);
    }
  }
}