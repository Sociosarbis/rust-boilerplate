use super::Solution;

impl Solution {
  pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
    let mut groups: Vec<Vec<i32>> = vec![vec![]; m as usize];
    let mut outer: Vec<i32> = vec![];
    let a = (n as usize) + (m as usize);
    let mut ret: Vec<i32> = vec![];
    let mut tp: Vec<Vec<i32>> = vec![vec![]; a];
    for i in 0..(n as usize) {
      if !before_items[i].is_empty() {
        let item = &before_items[i];
        let grp1 = group[i];
        for j in 0..item.len() {
          let grp2 = group[item[j] as usize];
          if grp1 != -1 {
            if grp2 != -1 && grp2 != grp1 {
              tp[(grp1 + n) as usize].push(grp2 + n);
            } else if grp2 == -1 {
              tp[(grp1 + n) as usize].push(item[j]);
            } else {
              tp[i].push(item[j]);
            }
          } else {
            if grp2 != -1 {
              tp[i].push(grp2 + n);
            } else {
              tp[i].push(item[j]);
            }
          }
        }
      }
    }

    let mut has_add: Vec<bool> = vec![false; a];
    let mut visited: Vec<bool> = vec![false;a];
    for i in 0..a {
      if i >= n as usize || group[i] == -1 {
        if !Solution::sort_items_dfs(i as i32, &mut outer, &mut has_add, &mut tp, &mut visited) {
            return ret;
        }
      } else {
        if !Solution::sort_items_dfs(i as i32, &mut groups[group[i] as usize], &mut has_add, &mut tp, &mut visited) {
          return ret;
        }
      }
    }

    for item in outer {
      if item >= n {
        ret.append(&mut groups[(item - n) as usize]);
      } else {
        ret.push(item);
      }
    }
    ret
  }

  pub fn sort_items_dfs(i: i32, new_group: &mut Vec<i32>, has_add: &mut Vec<bool>, tp: &mut Vec<Vec<i32>>, visited: &mut Vec<bool>) -> bool {
    if !has_add[i as usize] {
      if visited[i as usize] {
        return false;
      }
      visited[i as usize] = true;
      if !tp[i as usize].is_empty() {
        for item in tp[i as usize].to_owned() {
          if !has_add[item as usize] {
              if !Solution::sort_items_dfs(item, new_group, has_add, tp, visited) {
                return false;
              }
          }
        }
      }
      visited[i as usize] = false;
      if !has_add[i as usize] {
        has_add[i as usize] = true;
        new_group.push(i);
      }
    }
    return true;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    m: i32,
    group: Vec<i32>,
    before_items: Vec<Vec<i32>>,
    ret: Vec<i32>,
  }

  #[test]
  fn sort_items_simple() {
    let suites: Vec<Suite> = vec![
      Suite {
        n: 8,
        m: 2,
        group: vec![-1, -1, 1, 0, 0, 1, 0, -1],
        before_items: vec![
          vec![],
          vec![6],
          vec![5],
          vec![6],
          vec![3, 6],
          vec![],
          vec![],
          vec![],
        ],
        ret: vec![0, 6, 3, 4, 1, 7, 5, 2],
      },
      Suite {
        n: 8,
        m: 2,
        group: vec![-1, -1, 1, 0, 0, 1, 0, -1],
        before_items: vec![
          vec![],
          vec![6],
          vec![5],
          vec![6],
          vec![3],
          vec![],
          vec![4],
          vec![],
        ],
        ret: vec![],
      },
    ];

    for s in suites {
      assert_eq!(
        Solution::sort_items(s.n, s.m, s.group, s.before_items),
        s.ret
      );
    }
  }
}
