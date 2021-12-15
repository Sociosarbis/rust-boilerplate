use super::*;

impl Solution {
  pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
    let mut graph = vec![vec![];quiet.len()];
    let mut rev_graph = vec![vec![];quiet.len()];
    let mut count = vec![0;quiet.len()];
    for item in richer {
      graph[item[1] as usize].push(item[0] as usize);
      rev_graph[item[0] as usize].push(item[1] as usize);
      count[item[1] as usize] += 1;
    }
    let mut queue = vec![];
    for i in 0..count.len() {
      if count[i] == 0 {
        queue.push(i);
      }
    }
    let mut ret = vec![0;quiet.len()];
    while let Some(index) = queue.pop() {
      let mut min_i = index;
      for &item in &graph[index] {
        if quiet[ret[item] as usize] < quiet[min_i] {
          min_i = ret[item] as usize;
        }
      }
      for &item in &rev_graph[index] {
        count[item] -= 1;
        if count[item] == 0 {
          queue.push(item);
        }
      }
      ret[index] = min_i as i32;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    richer: Vec<Vec<i32>>,
    quiet: Vec<i32>,
    ret: Vec<i32>
  }

  #[test]
  fn test_loud_and_rich_simple() {
    let suites = vec![
      Suite {
        richer: t2_i![[1,0],[2,1],[3,1],[3,7],[4,3],[5,3],[6,3]],
        quiet: vec![3,2,5,4,6,1,7,0],
        ret: vec![5,5,2,5,4,5,6,7]
      },
      Suite {
        richer: t2_i![],
        quiet: vec![0],
        ret: vec![0]
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::loud_and_rich(s.richer, s.quiet));
    }
  }
}