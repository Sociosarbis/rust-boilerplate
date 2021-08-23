use super::*;

impl Solution {
  pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
      let mut grid_to_group: Vec<Vec<Vec<usize>>> = vec![vec![vec![0,0];grid[0].len()];grid.len()];
      let mut groups: Vec<Vec<(usize, usize, usize)>> = vec![vec![]];
      let mut grid_chars: Vec<Vec<char>> = vec![];
      for i in 0..grid.len() {
        grid_chars.push(grid[i].chars().collect());
      }
      for i in 0..grid_chars.len() {
        let chars = &grid_chars[i];
        for j in 0..chars.len() {
          let p = chars[j];
          if i > 0 {
            let group_id = {
              if grid_chars[i - 1][j] == '/' {
                grid_to_group[i - 1][j][1]
              } else {
                grid_to_group[i - 1][j][0]
              }
            };
            if p == ' ' {
              groups[group_id].push((i, j, 0));
              groups[group_id].push((i, j, 1));
              grid_to_group[i][j][0] = group_id;
              grid_to_group[i][j][1] = group_id;
            } else if p == '/' {
              groups[group_id].push((i, j, 0));
              grid_to_group[i][j][0] = group_id;
            } else {
              groups[group_id].push((i, j, 1));
              grid_to_group[i][j][1] = group_id;
            }
          }
          if j > 0 {
            if p == ' ' {
              Solution::may_merge_group(&mut grid_to_group, &mut groups, (i, j - 1, 1), (i, j, 0));
              Solution::may_merge_group(&mut grid_to_group, &mut groups, (i, j - 1, 1), (i, j, 1));
            } else {
              Solution::may_merge_group(&mut grid_to_group, &mut groups, (i, j - 1, 1), (i, j, 0));
            }
          }
          
          if p == ' ' {
            if grid_to_group[i][j][0] == 0 {
              let group_id = groups.len();
              grid_to_group[i][j][0] = group_id;
              grid_to_group[i][j][1] = group_id;
              groups.push(vec![(i, j, 0), (i, j, 1)]);
            }
          } else {
            if grid_to_group[i][j][0] == 0 {
              let group_id = groups.len();
              grid_to_group[i][j][0] = group_id;
              groups.push(vec![(i, j, 0)]);
            }
            if grid_to_group[i][j][1] == 0 {
              let group_id = groups.len();
              grid_to_group[i][j][1] = group_id;
              groups.push(vec![(i, j, 1)]);
            }
          }
          
        }
      }
      groups.into_iter().fold(0, |acc, item| { acc + if item.is_empty() { 0 } else { 1 }})
  }

  pub fn may_merge_group(grid_to_group: &mut Vec<Vec<Vec<usize>>>, groups: &mut Vec<Vec<(usize, usize, usize)>>, target: (usize, usize, usize), source: (usize, usize, usize)) {
    let group_id = grid_to_group[target.0][target.1][target.2];
    let old_group_id = grid_to_group[source.0][source.1][source.2];
    if old_group_id == 0 {
      groups[group_id].push(source);
      grid_to_group[source.0][source.1][source.2] = group_id;
    } else if group_id != old_group_id {
      for g in groups[old_group_id].to_owned() {
        grid_to_group[g.0][g.1][g.2] = group_id;
        groups[group_id].push(g);
      }
      groups[old_group_id].clear();
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    grid: Vec<String>,
    ret: i32
  }

  #[test]
  fn regions_by_slashes_simple() {
    let suites = vec![
      Suite {
        grid: t1![
          " /",
          "/ "
        ],
        ret: 2
      },
      Suite {
        grid: t1![
          " /",
          "  "
        ],
        ret: 1
      },
      Suite {
        grid: t1![
          "\\/",
          "/\\"
        ],
        ret: 4
      },
      Suite {
        grid: t1![
          "/\\",
          "\\/"
        ],
        ret: 5,
      },
      Suite {
        grid: t1![
          "//",
          "/ "
        ],
        ret: 3
      }
    ];

    for s in suites {
      assert_eq!(Solution::regions_by_slashes(s.grid), s.ret);
    }
  }
}