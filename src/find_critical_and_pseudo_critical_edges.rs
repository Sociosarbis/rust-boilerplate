use super::Solution;

impl Solution {
  /** 先获取最小生成树的cost，然后根据定义判断各条边是否符合要求 */
  pub fn find_critical_and_pseudo_critical_edges(n: i32, mut edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ret = vec![vec![], vec![]];
    for i in 0..edges.len() {
      edges[i].push(i as i32);
    }
    edges.sort_unstable_by(|a, b| { a[2].cmp(&b[2]) });
    let mut i_to_group = vec![0;n as usize];
    let mut groups: Vec<(i32, Vec<usize>)> = vec![(0, vec![])];
    let min_cost = {
      Solution::find_critical_and_pseudo_critical_edges_search(n as usize, &edges, &mut i_to_group, &mut groups, -1)
    };
    for i in 0..edges.len() {
      let e = &edges[i];
      let l = e[0] as usize;
      let r = e[1] as usize;
      let w = e[2];
      let mut cost = {
        let mut i_to_group = vec![0;n as usize];
        let mut groups: Vec<(i32, Vec<usize>)> = vec![(0, vec![])];
        Solution::find_critical_and_pseudo_critical_edges_search(n as usize, &edges, &mut i_to_group, &mut groups, i as i32)
      };
      if cost > min_cost || cost == 0 {
        ret[0].push(e[3]);
      } else {
        let mut i_to_group = vec![0;n as usize];
        let mut groups: Vec<(i32, Vec<usize>)> = vec![(0, vec![])];
        i_to_group[l] = 1;
        i_to_group[r] = 1;
        groups.push((w, vec![l, r]));
        cost = {
          Solution::find_critical_and_pseudo_critical_edges_search(n as usize, &edges, &mut i_to_group, &mut groups, i as i32)
        };
        if cost == min_cost {
          ret[1].push(e[3]);
        }
      }
    }
    ret
  }

  fn find_critical_and_pseudo_critical_edges_search(n: usize, edges: &Vec<Vec<i32>>, i_to_group: &mut Vec<usize>, groups: &mut Vec<(i32, Vec<usize>)>, skip: i32) -> i32 {
    let mut cost = 0;
    for i in 0..edges.len() {
      if i as i32 == skip { continue }
      let e = &edges[i];
      let l = e[0] as usize;
      let r = e[1] as usize;
      let w = e[2];
      let mut group_id = i_to_group[l];
      if i_to_group[l] == 0 {
        if i_to_group[r] == 0 {
          group_id = groups.len();
          groups.push((w, vec![l, r]));
          i_to_group[l] = group_id;
          i_to_group[r] = group_id;
        } else {
          group_id = i_to_group[r];
          i_to_group[l] = group_id;
          groups[group_id].1.push(l);
          groups[group_id].0 += w;
        }
      } else {
        if i_to_group[r] == 0 {
          i_to_group[r] = group_id;
          groups[group_id].1.push(r);
          groups[group_id].0 += w;
        } else if i_to_group[l] != i_to_group[r] {
          let old_group_id = i_to_group[r];
          for item in groups[old_group_id].1.to_owned() {
            groups[group_id].1.push(item);
            i_to_group[item] = group_id;
          }
          groups[group_id].0 += groups[old_group_id].0 + w;
          groups[old_group_id].1.clear();
        }
      }
      if groups[group_id].1.len() == n as usize {
        cost = groups[group_id].0;
        break;
      }
    }
    cost
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    edges: Vec<Vec<i32>>,
    ret: Vec<Vec<i32>>
  }

  #[test]
  fn find_critical_and_pseudo_critical_edges_simple() {
    let suites = vec![
      Suite {
        n: 5,
        edges: Solution::t2_i(vec![&[0,1,1],&[1,2,1],&[2,3,2],&[0,3,2],&[0,4,3],&[3,4,3],&[1,4,6]]),
        ret: Solution::t2_i(vec![&[0,1],&[2,3,4,5]])
      },
      Suite {
        n: 4,
        edges: Solution::t2_i(vec![&[0,1,1],&[1,2,1],&[2,3,1],&[0,3,1]]),
        ret: Solution::t2_i(vec![&[],&[0,1,2,3]])
      },
      Suite {
        n: 6,
        edges: Solution::t2_i(vec![&[0,1,1],&[1,2,1],&[0,2,1],&[2,3,4],&[3,4,2],&[3,5,2],&[4,5,2]]),
        ret: Solution::t2_i(vec![&[3],&[0,1,2,4,5,6]])
      }
    ];

    for s in suites {
      assert_eq!(Solution::find_critical_and_pseudo_critical_edges(s.n, s.edges), s.ret);
    }
  }
}