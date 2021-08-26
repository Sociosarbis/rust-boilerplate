use super::*;

impl Solution {
  // 最小生成树
  pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let mut edges: Vec<(i32, usize, usize)> = vec![];
    let mut point_to_group: Vec<usize> = vec![0;points.len()];
    let mut groups: Vec<(i32, Vec<usize>)> = vec![(0, vec![])];
    let n = points.len();
    for i in 0..points.len() {
      for j in i+1..points.len() {
        let dist = (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
        edges.push((dist, i, j));
      }
    }
    edges.sort_by(|a, b| {
      a.0.cmp(&b.0)
    });
    for e in edges {
      let mut group_id = point_to_group[e.1];
      if point_to_group[e.1] == 0 {
        if point_to_group[e.2] == 0 {
          group_id = groups.len();
          point_to_group[e.1] = group_id;
          point_to_group[e.2] = group_id;
          groups.push((e.0, vec![e.1, e.2]));
        } else {
          group_id = point_to_group[e.2];
          point_to_group[e.1] = group_id;
          groups[group_id].0 += e.0;
          groups[group_id].1.push(e.1);
        }
      } else {
        if point_to_group[e.2] == 0 {
          point_to_group[e.2] = group_id;
          groups[group_id].0 += e.0;
          groups[group_id].1.push(e.2);
          if groups[group_id].1.len() == n {
            return groups[group_id].0
          }
        } else {
          let old_group_id = point_to_group[e.2];
          if old_group_id != group_id {
            for m in groups[old_group_id].1.to_owned() {
              groups[group_id].1.push(m);
              point_to_group[m] = group_id;
            }
            groups[old_group_id].1.clear();
            groups[group_id].0 += groups[old_group_id].0 + e.0;
            groups[old_group_id].0 = 0;
          }
        }
      }
      if groups[group_id].1.len() == n {
        return groups[group_id].0
      }
    }
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    points: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn min_cost_connect_points_simple() {
    let suites = vec![
      Suite {
        points: Solution::t2_i(vec![&[0,0],&[2,2],&[3,10],&[5,2],&[7,0]]),
        ret: 20
      },
      Suite {
        points: Solution::t2_i(vec![&[3,12],&[-2,5],&[-4,1]]),
        ret: 18,
      },
      Suite {
        points: Solution::t2_i(vec![&[0,0],&[1,1],&[1,0],&[-1,1]]),
        ret: 4
      },
      Suite {
        points: Solution::t2_i(vec![&[-1000000,-1000000],&[1000000,1000000]]),
        ret: 4000000
      }
    ];

    for s in suites {
      assert_eq!(Solution::min_cost_connect_points(s.points), s.ret);
    }
  }
}