use super::Solution;

impl Solution {
  pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ancestors: Vec<i32> = vec![0;edges.len() + 1];
    let mut groups: Vec<Vec<i32>> = vec![vec![];edges.len() + 1];
    for e in edges {
      let l = e[0] as usize;
      let r = e[1] as usize;
      if ancestors[r] != 0 && ancestors[l] != 0 && ancestors[r] == ancestors[l] {
        return e;
      }
      if ancestors[l] == 0 {
        if ancestors[r] == 0 {
          ancestors[l] = l as i32;
          ancestors[r] = l as i32;
          groups[l].push(l as i32);
          groups[l].push(r as i32);
        } else {
          ancestors[l] = ancestors[r] as i32;
          groups[ancestors[r] as usize].push(l as i32);
        }
      } else  {
        if ancestors[r] == 0 {
          ancestors[r] = ancestors[l];
          groups[ancestors[l] as usize].push(r as i32);
        } else {
          let source = if groups[ancestors[l] as usize].len() < groups[ancestors[r] as usize].len() { ancestors[l] } else { ancestors[r] };
          let target = if source == ancestors[l] { ancestors[r] } else { ancestors[l] };
          let source_group = groups[source as usize].to_owned();
          for i in 0..source_group.len() {
            ancestors[source_group[i] as usize] = target;
            groups[target as usize].push(source_group[i]);
          }
          groups[source as usize].clear();
        }
      }
    }
    vec![]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    edges: Vec<Vec<i32>>,
    ret: Vec<i32>
  }

  #[test]
  fn find_redundant_connection_simple() {
    let suites: Vec<Suite> = vec![
      Suite {
        edges: vec![vec![1,2], vec![1,3], vec![2,3]],
        ret: vec![2,3]
      },
      Suite {
        edges: vec![vec![1,2], vec![2,3], vec![3,4], vec![1,4], vec![1,5]],
        ret: vec![1,4]
      }
    ];

    for s in suites {
      assert_eq!(Solution::find_redundant_connection(s.edges), s.ret);
    }
  }
}