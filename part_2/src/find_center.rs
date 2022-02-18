use super::*;

impl Solution {
  pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    if edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1] {
      edges[0][0]
    } else {
      edges[0][1]
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    edges: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn test_find_center_simple() {
    let suites = vec![
      Suite {
        edges: t2_i![[1,2],[2,3],[4,2]],
        ret: 2
      },
      Suite {
        edges: t2_i![[1,2],[5,1],[1,3],[1,4]],
        ret: 1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_center(s.edges));
    }
  }

}