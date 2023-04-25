use super::*;

impl Solution {
  pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut names: Vec<(usize, String)> = names.into_iter().enumerate().map(|(i, name)| (i, name)).collect();
    names.sort_unstable_by(|a, b| heights[b.0].cmp(&heights[a.0]));
    names.into_iter().map(|item| item.1).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    names: Vec<String>,
    heights: Vec<i32>,
    ret: Vec<String>
  }

  #[test]
  fn test_sort_people_simple() {
    let suites = vec![
      Suite {
        names: t1!["Mary","John","Emma"],
        heights: vec![180,165,170],
        ret: t1!["Mary","Emma","John"]
      },
      Suite {
        names: t1!["Alice","Bob","Bob"],
        heights: vec![155,185,150],
        ret: t1!["Bob","Alice","Bob"]
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::sort_people(s.names, s.heights));
    }
  }
}