use super::*;
use std::cmp::Ordering;

impl Solution {
  pub fn max_num_edges_to_remove(n: i32, mut edges: Vec<Vec<i32>>) -> i32 {
    edges.sort_unstable_by(|a, b| {
      if a[0] == 3 && b[0] != 3 {
        return Ordering::Less;
      } else if a[0] != 3 && b[0] == 3 {
        return Ordering::Greater;
      }
      Ordering::Equal
    });
    let mut i_to_group: [Vec<usize>;2] = [vec![0;n as usize + 1], vec![0;n as usize + 1]];
    let mut groups: [Vec<Vec<usize>>;2] = [vec![vec![]], vec![vec![]]];
    let mut unused_edges: [Vec<usize>;2] = [vec![], vec![]];
    let mut e = vec![false;edges.len()];
    for i in 0..2 {
      let m = &mut i_to_group[i];
      let g = &mut groups[i];
      for j in 0..edges.len() {
        let e = &edges[j];
        let t = e[0];
        if t == 3 || t == i as i32 + 1 {
          let l = e[1] as usize;
          let r = e[2] as usize;
          if m[l] == 0 {
            if m[r] == 0 {
              let group_id = g.len();
              m[l] = group_id;
              m[r] = group_id;
              g.push(vec![l, r]);
            } else {
              let group_id = m[r];
              m[l] = group_id;
              g[group_id].push(l);
            }
          } else {
            let group_id = m[l];
            if m[r] == 0 {
              m[r] = group_id;
              g[group_id].push(r);
            } else if m[r] != m[l] {
              let old_group_id = m[r];
              for member in g[old_group_id].to_owned() {
                g[group_id].push(member);
                m[member] = group_id;
              }
              g[old_group_id].clear();
            } else {
              unused_edges[i].push(j);
            }
          }
        }
      }
    }
    for i in 0..2 {
      let mut found = false;
      for j in 0..groups[i].len() {
        if groups[i][j].len() == n as usize {
          found = true;
          break;
        } else if !groups[i][j].is_empty() {
          return -1;
        }
      }
      if !found {
        return -1;
      }
    }
    let mut ret = 0;
    for i in 0..2 {
      for j in 0..unused_edges[i].len() {
        let index = unused_edges[i][j];
        if !e[index] {
          e[index] = true;
          ret +=1;
        }
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    edges: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn max_num_edges_to_remove_simple() {
    let suites = vec![
      Suite {
        n: 4,
        edges: Solution::t2_i(vec![&[3,1,2],&[3,2,3],&[1,1,3],&[1,2,4],&[1,1,2],&[2,3,4]]),
        ret: 2
      },
      Suite {
        n: 4,
        edges: Solution::t2_i(vec![&[3,1,2],&[3,2,3],&[1,1,4],&[2,1,4]]),
        ret: 0
      },
      Suite {
        n: 4,
        edges: Solution::t2_i(vec![&[3,2,3],&[1,1,2],&[2,3,4]]),
        ret: -1
      }
    ];

    for s in suites {
      assert_eq!(Solution::max_num_edges_to_remove(s.n, s.edges), s.ret);
    }
  }
}