use super::*;

impl Solution {
  pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
    let mut neighbors = vec![vec![];n as usize];
    for edge in edges {
      let left = edge[0] as usize - 1;
      let right  = edge[1] as usize - 1;
      neighbors[left].push(right);
      neighbors[right].push(left);
    }
    let mut visited = vec![None;n as usize];
    visited[0] = Some((0, false));
    let mut queue = vec![];
    queue.push(0);
    let mut paths = 0;
    let mut found = false;
    while !queue.is_empty() {
      paths += 1;
      let length = queue.len();
      for i in 0..length {
        for &next in &neighbors[queue[i]] {
          if !found && next + 1 == neighbors.len() {
            found = true;
          }
          if let Some(record) = visited[next].as_mut() {
            if !record.1 && record.0 + 1 == paths {
              record.1 = true;
            }
          } else {
            visited[next] = Some((paths, false));
            queue.push(next);
          }
        }
      }
      if found {
        let has_one_more = {
          let mut current = paths - 1;
          let mut queue = vec![n as usize - 1];
          let mut found = false;
          while !queue.is_empty() {
            let length = queue.len();
            for i in 0..length {
              for &next in &neighbors[queue[i]] {
                if let Some(record) = visited[next] {
                  if (record.0 == current && record.1) || record.0 == current + 1 {
                    found = true;
                    break;
                  }
                  if record.0 == current {
                    queue.push(next);
                  }
                }
              }
            }
            current -= 1;
            queue.drain(..length);
          }
          found
        };
        paths += if has_one_more { 1 } else { 2 };
        let mut ret = 0;
        for _ in 0..paths {
          if (ret / change) & 1 != 0 {
            ret += change - ret % change;
          }
          ret += time;
        }
        return ret;
      }
      queue.drain(..length);
    }
    unreachable!()
  }
}


#[cfg(test)]
mod test {
  use super::*;

  struct Suite {
    n: i32,
    edges: Vec<Vec<i32>>,
    time: i32,
    change: i32,
    ret: i32
  }

  #[test]
  fn test_second_minimum_simple() {
    let suites = vec![
      Suite {
        n: 5,
        edges: t2_i![[1,2],[1,3],[1,4],[3,4],[4,5]],
        time: 3,
        change: 5,
        ret: 13
      },
      Suite {
        n: 2,
        edges: t2_i![[1,2]],
        time: 3,
        change: 2,
        ret: 11
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::second_minimum(s.n, s.edges, s.time, s.change));
    }
  }
}