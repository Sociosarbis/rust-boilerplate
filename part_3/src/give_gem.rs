use super::*;

impl Solution {
  pub fn give_gem(mut gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
    for op in operations {
      let x = op[0] as usize;
      let y = op[1] as usize;
      gem[y] += gem[x] / 2;
      gem[x] -= gem[x] / 2;
    }
    *gem.iter().max().unwrap() - *gem.iter().min().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    gem: Vec<i32>,
    operations: Vec<Vec<i32>>,
    ret: i32,
  }

  #[test]
  fn test_give_gem_simple() {
    let suites = vec![
      Suite {
        gem: vec![3, 1, 2],
        operations: t2_i![[0, 2], [2, 1], [2, 0]],
        ret: 2,
      },
      Suite {
        gem: vec![100, 0, 50, 100],
        operations: t2_i![[0, 2], [0, 1], [3, 0], [3, 0]],
        ret: 75,
      },
      Suite {
        gem: vec![0, 0, 0, 0],
        operations: t2_i![[1, 2], [3, 1], [1, 2]],
        ret: 0,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::give_gem(s.gem, s.operations));
    }
  }
}
