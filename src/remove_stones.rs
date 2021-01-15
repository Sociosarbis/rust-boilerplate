use super::Solution;


impl Solution {
  pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    let mut groups: Vec<Vec<usize>> = vec![vec![]];
    let mut rows: Vec<usize> = vec![];
    let mut cols: Vec<usize> = vec![];
    for i in 0..stones.len() {
      let x = stones[i][0] as usize;
      let y = stones[i][1] as usize;
      if y >= rows.len() {
        rows.resize(y + 1, 0);
      }
      if x >= cols.len() {
        cols.resize(x + 1, 0);
      }
      if rows[y] == 0 {
        if cols[x] == 0 {
          rows[y] = groups.len();
          cols[x] = groups.len();
          groups.push(vec![i]);
        } else {
          rows[y] = cols[x];
          groups[cols[x]].push(i);
        }
      } else {
        if cols[x] == 0 {
          cols[x] = rows[y];
          groups[rows[y]].push(i);
        } else {
          if cols[x] != rows[y] {
            let g1 = rows[y];
            let g2 = cols[x];
            let t = if groups[g1].len() >= groups[g2].len() { g1 } else { g2 };
            let s = if t == g1 { g2 } else { g1 };
            for m in groups[s].to_owned() {
              groups[t].push(m);
              rows[stones[m][1] as usize] = t;
              cols[stones[m][0] as usize] = t;
            }
            groups[s].clear();
          }
          groups[cols[x]].push(i);
        }
      }
    }
    let mut group_num = 0;
    for g in groups {
      if !g.is_empty() {
        group_num += 1;
      }
    }
    stones.len() as i32 - group_num
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    stones: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn remove_stones_simple() {
    let suites = vec![
      Suite {
        stones: vec![vec![0,0],vec![0,1],vec![1,0],vec![1,2],vec![2,1],vec![2,2]],
        ret: 5
      },
      Suite {
        stones: vec![vec![0,0],vec![0,2],vec![1,1],vec![2,0],vec![2,2]],
        ret: 3
      },
      Suite {
        stones: vec![vec![0,0]],
        ret: 0
      },
      Suite {
        stones: vec![vec![0,1],vec![1,0],vec![1,1]],
        ret: 2
      }
    ];

    for s in suites {
      assert_eq!(Solution::remove_stones(s.stones), s.ret);
    }
  }
}