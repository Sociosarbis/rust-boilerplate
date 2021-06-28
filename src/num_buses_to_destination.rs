use super::Solution;

impl Solution {
  pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    if source == target {
      return 0;
    }
    let mut dest_to_route = vec![vec![];1000000];
    for i in 0..routes.len() {
      for &dest in &routes[i] {
        dest_to_route[dest as usize].push(i);
      }
    }
    let mut count = 0;
    let mut visited = vec![false;routes.len()];
    let mut queue = vec![source];
    while !queue.is_empty() {
      count += 1;
      let n = queue.len();
      for i in 0..n {
        for &j in &dest_to_route[queue[i] as usize] {
          if !visited[j] {
            visited[j] = true;
            for &k in &routes[j] {
              if k == target {
                return count;
              }
              queue.push(k);
            }
          }
        }
        dest_to_route[queue[i] as usize].clear();
      }
      queue.drain(0..n);
    }
    -1
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    routes: Vec<&'a [i32]>,
    source: i32,
    target: i32,
    ret: i32
  }

  #[test]
  fn test_num_buses_to_destination_simple() {
    let suites = vec![
      Suite {
        routes: vec![&[1,2,7],&[3,6,7]],
        source: 1,
        target: 6,
        ret: 2
      },
      Suite {
        routes: vec![&[7,12],&[4,5,15],&[6],&[15,19],&[9,12,13]],
        source: 15,
        target: 12,
        ret: -1
      }
    ];

    for s in suites {
      assert_eq!(Solution::num_buses_to_destination(Solution::t2_i(s.routes), s.source, s.target), s.ret);
    }
  }
}