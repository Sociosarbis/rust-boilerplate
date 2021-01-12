use super::Solution;

impl Solution {
  pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
    let mut groups: Vec<Vec<i32>> = vec![vec![]; m as usize];
    let mut outer: Vec<i32> = vec![];
    for i in 0..(n as usize) {
      if group[i] != -1 {
        groups[group[i] as usize].push(i as i32);
      }
    }
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
    for i in 0..a {
      if !has_add[i] {
        if !tp[i].is_empty() {
          if i >= n as usize || group[i] == -1 {
            let mut bfs: Vec<i32> = tp[i].to_owned();
            let mut visited: Vec<bool> = vec![false; a];
            visited[i] = true;
            outer.push(i as i32);
            has_add[i] = true;
            while !bfs.is_empty() {
              let cur = bfs.remove(0);
              if visited[cur as usize] {
                return ret;
              }
              if !has_add[cur as usize] {
                if !tp[cur as usize].is_empty() {
                  for item in &tp[cur as usize] {
                    if !has_add[*item as usize] {
                      bfs.push(*item);
                    }
                  }
                }
                visited[cur as usize] = true;
                outer.push(cur);
                has_add[cur as usize] = true;
              }
            }
          } else {
            let mut new_group = vec![];
            let g = &groups[group[i] as usize];
            for j in 0..g.len() {
              let item = g[j];
              println!("{:?}:{:?}:{:?}", item, g, new_group);
              if !has_add[item as usize] {
                if !tp[item as usize].is_empty() {
                  let mut bfs: Vec<i32> = tp[item as usize].to_owned();
                  let mut visited: Vec<bool> = vec![false; a];
                  visited[item as usize] = true;
                  new_group.push(item);
                  has_add[item as usize] = true;
                  while !bfs.is_empty() {
                    let cur = bfs.remove(0);
                    if visited[cur as usize] {
                      return ret;
                    }
                    if !has_add[cur as usize] {
                      if !tp[cur as usize].is_empty() {
                        for item in &tp[cur as usize] {
                          if !has_add[*item as usize] {
                            bfs.push(*item);
                          }
                        }
                      }
                      println!("{:?}", has_add[cur as usize]);
                      visited[cur as usize] = true;
                      new_group.push(cur);
                      has_add[cur as usize] = true;
                    }
                  }
                } else {
                  has_add[item as usize] = true;
                  new_group.push(item);
                }
              }
            }
            groups[group[i] as usize] = new_group;
          }
        } else if i >= n as usize || group[i] == -1 {
          has_add[i] = true;
          outer.push(i as i32);
        }
      }
    }

    println!("{:?}:{:?}", outer, groups);

    for item in outer {
      if item >= n {
        ret.append(&mut groups[(item - n) as usize]);
      } else {
        ret.push(item);
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
        ret: vec![6, 3, 4, 1, 5, 2, 0, 7],
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
