use super::Solution;
use std::collections::hash_map::HashMap;
use std::cell::RefCell;


impl Solution {

  pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
    let mut graph: HashMap<&String, RefCell<HashMap<String, f64>>> = HashMap::new();
    let mut rel: HashMap<&String, RefCell<String>> = HashMap::new();
    let mut ret: Vec<f64> = vec![];
    for i in 0..equations.len() {
      let s = &equations[i][0];
      let t = &equations[i][1];
      if let Some(g1) = rel.get(s) {
        if let Some(g2) = rel.get(t) {
          if g2.borrow().to_owned() != g1.borrow().to_owned() {
            {
              let mut m1 = graph.get(&g1.borrow().to_owned()).unwrap().borrow_mut();
              let m2 = graph.get(&g2.borrow().to_owned()).unwrap().borrow().to_owned();
              let base = m1.get(s).unwrap() * values[i] / m2.get(t).unwrap();
              for p in m2 {
                *rel.get(&p.0).unwrap().borrow_mut() = g1.borrow().to_owned();
                m1.insert(p.0, p.1 * base);
              }
            }
          }
          graph.remove(t);
          continue;
        }
      }
      let g2 = {
        if rel.contains_key(t) {
          rel.get(t).unwrap().borrow().to_owned()
        } else {
          String::new()
        }
      };
      if g2.len() != 0 {
        rel.insert(s, RefCell::new(g2.to_owned()));
        let val = {
          *graph.get(&g2).unwrap().borrow().get(t).unwrap() / values[i]
        };
        graph.get(&g2).unwrap().borrow_mut().insert(s.to_owned(), val);
      } else {
        let g1 = {
          if !rel.contains_key(s) {
            rel.insert(s, RefCell::new(s.to_owned()));
            let mut new_map: HashMap<String, f64> = HashMap::new();
            new_map.insert(s.to_owned(), 1.0);
            graph.insert(s, RefCell::new(new_map));
            s.to_owned()
          } else {
            rel.get(s).unwrap().borrow().to_owned()
          }
        };
        rel.insert(t, RefCell::new(g1.to_owned()));
        let val = {
          *graph.get(&g1).unwrap().borrow().get(s).unwrap() * values[i]
        };
        graph.get(&g1).unwrap().borrow_mut().insert(t.to_owned(), val);
      }
    }
    for q in queries {
      let s = &q[0];
      let t = &q[1];
      if rel.contains_key(s) && rel.contains_key(t) {
        if let Some(g1) = rel.get(s) {
          if let Some(g2) = rel.get(t) {
            if g1.borrow().to_owned() == g2.borrow().to_owned() {
              let g = g1.borrow().to_owned();
              ret.push(graph.get(&g).unwrap().borrow().get(t).unwrap() / graph.get(&g).unwrap().borrow().get(s).unwrap());
              continue;
            }
          }
        }
      }
      ret.push(-1.0);
    }
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
      Suite {
        equations: t(&[&["a","e"],&["b","e"]]),
        values: vec![4.0,3.0],
        queries: t(&[&["a","b"],&["e","e"],&["x","x"]]),
        ret: vec![1.3333333333333333,1.00000,-1.00000]
      },
      Suite {
        equations: t(&[&["a","b"],&["e","f"],&["b","e"]]),
        values: vec![3.4,1.4,2.3],
        queries: t(&[&["b","a"],&["a","f"],&["f","f"],&["e","e"],&["c","c"],&["a","c"],&["f","e"]]),
        ret: vec![0.29411764705882354, 10.947999999999999, 1.0, 1.0, -1.0, -1.0, 0.7142857142857143]
      },
      Suite {
        equations: t(&[&["a","b"],&["b","c"],&["a","c"]]),
        values: vec![2.0,3.0,6.0],
        queries: t(&[&["a","c"],&["b","a"],&["a","e"],&["a","a"],&["x","x"]]),
        ret: vec![6.00000,0.50000,-1.00000,1.00000,-1.00000]
      }
    ];

    for s in suites {
      assert_eq!(Solution::calc_equation(s.equations, s.values, s.queries), s.ret);
    }
  }
}