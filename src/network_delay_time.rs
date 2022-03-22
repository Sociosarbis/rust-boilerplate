use super::*;

impl Solution {
  pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let mut num_to_edges: Vec<Vec<Vec<i32>>> = vec![vec![];n as usize + 1];
    let mut rest = n;
    let mut visited = vec![false;n as usize + 1];
    for time in times {
      num_to_edges[time[0] as usize].push(time);
    }
    let mut edges_to_visit: Vec<Vec<i32>> = vec![];
    let mut cur_edge = vec![k, 0];
    let mut ret = 0;
    while !cur_edge.is_empty() {
      if !visited[cur_edge[0] as usize] {
        visited[cur_edge[0] as usize] = true; 
        rest -= 1;
        if ret < cur_edge[1] {
          ret = cur_edge[1];
        }
        if rest == 0 {
          break;
        }
        for edge in &num_to_edges[cur_edge[0] as usize] {
          if !visited[edge[1] as usize] {
            let time = edge[2] + cur_edge[1];
            if edges_to_visit.is_empty() {
              edges_to_visit.push(vec![edge[1], time]);
            } else {
              let mut l = 0;
              let mut r = edges_to_visit.len() - 1;
              while l <= r {
                let mid = (l + r) / 2;
                if edges_to_visit[mid][1] < time {
                  l = mid + 1;
                } else if edges_to_visit[mid][1] > time {
                  if mid > 0 {
                    r = mid - 1;
                  } else {
                    break;
                  }
                } else {
                  l = mid;
                  break;
                }
              }
              if l < edges_to_visit.len() {
                edges_to_visit.insert(l, vec![edge[1], time]);
              } else {
                edges_to_visit.push(vec![edge[1], time]);
              }
            }
          }
        }
      }
      if edges_to_visit.is_empty() {
        cur_edge = vec![];
      } else {
        cur_edge = edges_to_visit.remove(0);
      }
    }
    if rest == 0 {
      ret
    } else {
      -1
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    times: Vec<Vec<i32>>,
    n: i32,
    k: i32,
    ret: i32
  }

  #[test]
  fn test_network_delay_time_simple() {
    let suites = vec![
      Suite {
        times: t2_i![[2,1,1],[2,3,1],[3,4,1]],
        n: 4,
        k: 2,
        ret: 2
      },
      Suite {
        times: t2_i![[1,2,1]],
        n: 2,
        k: 1,
        ret: 1
      },
      Suite {
        times: t2_i![[1,2,1]],
        n: 2,
        k: 2,
        ret: -1
      }
    ];
    
    for s in suites {
      assert_eq!(s.ret, Solution::network_delay_time(s.times, s.n, s.k));
    }
  }
}