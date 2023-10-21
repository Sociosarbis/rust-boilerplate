use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn count_pairs_2(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    let mut id_to_group = vec![0; n as usize];
    let mut groups = HashMap::new();
    let mut next_id = 1;
    for e in edges {
      let a = e[0];
      let b = e[1];
      let a_group = id_to_group[a as usize];
      let b_group = id_to_group[b as usize];
      if a_group == 0 {
        if b_group == 0 {
          groups.insert(next_id, vec![a, b]);
          id_to_group[a as usize] = next_id;
          id_to_group[b as usize] = next_id;
          next_id += 1;
        } else {
          id_to_group[a as usize] = b_group;
          if let Some(g) = groups.get_mut(&b_group) {
            g.push(a);
          }
        }
      } else if a_group != b_group {
        if b_group == 0 {
          id_to_group[b as usize] = a_group;
          if let Some(g) = groups.get_mut(&a_group) {
            g.push(b);
          }
        } else {
          if let Some(old_group) = groups.remove(&b_group) {
            if let Some(g) = groups.get_mut(&a_group) {
              for &id in &old_group {
                id_to_group[id as usize] = a_group;
              }
              g.extend(old_group);
            }
          }
        }
      }
    }
    let mut group_counts: Vec<i64> = groups
      .into_iter()
      .map(|(_, items)| items.len() as i64)
      .collect();
    let mut no_group_count = 0;
    for i in 0..n as usize {
      if id_to_group[i] == 0 {
        no_group_count += 1;
      }
    }
    let mut ret = if no_group_count > 1 {
      no_group_count * (no_group_count - 1) / 2
    } else {
      0
    };
    group_counts.push(no_group_count);
    for (i, c) in group_counts.iter().enumerate() {
      for j in i + 1..group_counts.len() {
        ret += c * group_counts[j];
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
    ret: i64,
  }

  #[test]
  fn test_count_pairs_simple() {
    let suites = vec![
      Suite {
        n: 3,
        edges: t2_i![[0, 1], [0, 2], [1, 2]],
        ret: 0,
      },
      Suite {
        n: 7,
        edges: t2_i![[0, 2], [0, 5], [2, 4], [1, 6], [5, 4]],
        ret: 14,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_pairs_2(s.n, s.edges));
    }
  }
}
