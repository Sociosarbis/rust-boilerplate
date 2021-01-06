use super::Solution;
use std::collections::hash_map::HashMap;

impl Solution {
  /*pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
    let mut graph: HashMap<String, HashMap<String, f64>> = HashMap::new();
    let mut ret: Vec<f64> = vec![];
    for i in 0..equations.len() {
      if !graph.contains_key(&equations[i][0]) {
        graph.insert(equations[i][0].to_owned(), HashMap::new());
      }
      if !graph.contains_key(&equations[i][1]) {
        graph.insert(equations[i][1].to_owned(), HashMap::new());
      }
      let r1 = graph.get_mut(&equations[i][0]).unwrap();
      r1.insert(equations[i][1].to_owned(), 1.0 / values[i]);
      let r2 = graph.get_mut(&equations[i][1]).unwrap();
      r2.insert(equations[i][0].to_owned(), values[i]);
    }
    let mut visited: HashMap<String, bool> = HashMap::new();
    for pair in &graph {
      visited.insert(pair.0.to_owned(), false);
    }
    let mut bfs: Vec<(&String, f64)> = vec![];
    let mut new_visited: HashMap<String, bool>; 
    for q in &queries {
      let s = &q[1];
      let t = &q[0];
      let n = ret.len();
      new_visited = visited.clone();
      if let Some(is_visited) = new_visited.get_mut(s) {
        *is_visited = true;
        bfs.push((s, 1.0));
        while !bfs.is_empty() {
          if let Some((cur, base)) = bfs.pop() {
            if let Some(res) = graph.get(cur) {
              if res.contains_key(t) {
                bfs.clear();
                ret.push(*(res.get(t).unwrap()) * base);
              } else {
                for it in res {
                  if let Some(v) = new_visited.get_mut(it.0) {
                    if *v == true {
                      continue
                    } else {
                      *v = true;
                    }
                  }
                  bfs.push((it.0, base * *it.1));
                }
              }
            }
          }
        }
      }
      if ret.len() == n {
        ret.push(-1.0);
      }
    }
    ret
  }*/

  pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
    let mut graph: HashMap<String, HashMap<String, f64>> = HashMap::new();
    let mut rel: HashMap<String, String> = HashMap::new();
    let mut ret: Vec<f64> = vec![];
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
    ret: Vec<f64>
  }

  fn t(arr_str: &[&[&str]]) -> Vec<Vec<String>> {
    arr_str.into_iter()
      .map(|&a| { 
        a.into_iter()
          .map(|&s| { 
            s.to_owned() 
          }).collect() 
      }).collect()
  }

  #[test]
  fn calc_equation_simple() {
    let suites = vec![
      Suite{
        equations: t(&[&["a","b"],&["b","c"]]),
        values: vec![2.0,3.0],
        queries: t(&[&["a","c"],&["b","a"],&["a","e"],&["a","a"],&["x","x"]]),
        ret: vec![6.00000,0.50000,-1.00000,1.00000,-1.00000]
      },
      Suite{
        equations: t(&[&["a","b"],&["b","c"],&["bc","cd"]]),
        values: vec![1.5,2.5,5.0],
        queries: t(&[&["a","c"],&["c","b"],&["bc","cd"],&["cd","bc"]]),
        ret: vec![3.75000,0.40000,5.00000,0.20000]
      },
      Suite{
        equations: t(&[&["a","b"]]),
        values: vec![0.5],
        queries: t(&[&["a","b"],&["b","a"],&["a","c"],&["x","y"]]),
        ret: vec![0.50000,2.00000,-1.00000,-1.00000]
      },
    ];

    for s in suites {
      assert_eq!(Solution::calc_equation(s.equations, s.values, s.queries), s.ret);
    }
  }
}