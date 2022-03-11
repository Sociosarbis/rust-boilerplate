use super::*;

fn dfs(tree: &Vec<Vec<usize>>, i: usize, counter: &mut Vec<i32>) -> i32 {
  if counter[i] == 0 {
    counter[i] = 1 + tree[i].iter().fold(0, |acc, &j| acc + dfs(tree, j, counter));
  }
  counter[i]
}

impl Solution {
  pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
    let mut tree = vec![vec![];parents.len()];
    for i in 0..parents.len() {
      if i != 0 {
        tree[parents[i] as usize].push(i);
      }
    }
    let mut counter = vec![0;parents.len()];
    for i in 0..parents.len() {
      dfs(&tree, i, &mut counter);
    }
    let mut max = 0;
    let mut ret = 0;
    let n = counter[0];
    for i in 0..parents.len() {
      let mut temp = 1;
      if i != 0 {
        temp *= (n - counter[i]) as i64;
      }
      for &j in &tree[i] {
        temp *= counter[j] as i64;
      }
      if temp > max {
        ret = 1;
        max = temp;
      } else if temp == max {
        ret += 1;
      }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    parents: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_count_highest_score_nodes_simple() {
    let suites = vec![
      Suite {
        parents: vec![-1,2,0,2,0],
        ret: 3
      },
      Suite {
        parents: vec![-1,2,0],
        ret: 2
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_highest_score_nodes(s.parents));
    }
  }
}