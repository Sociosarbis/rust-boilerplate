use super::*;

impl Solution {
  pub fn find_replace_string(
    s: String,
    indices: Vec<i32>,
    sources: Vec<String>,
    targets: Vec<String>,
  ) -> String {
    let mut indices_ptrs: Vec<(usize, usize)> = indices
      .into_iter()
      .enumerate()
      .map(|(ptr, i)| (i as usize, ptr))
      .collect();
    indices_ptrs.sort_unstable();
    let mut ret = String::new();
    let mut i = 0;
    let mut j = 0;
    let mut chars = s.chars();
    'a: while let Some(mut c1) = chars.next() {
      if j < indices_ptrs.len() && i == indices_ptrs[j].0 {
        let index = indices_ptrs[j].1;
        j += 1;
        let mut chars2 = sources[index].chars();
        let c2 = chars2.next().unwrap();
        if c2 != c1 {
          ret.push(c1);
        } else {
          let mut temp = String::new();
          temp.push(c1);
          while let Some(c2) = chars2.next() {
            c1 = chars.next().unwrap_or('\0');
            if c1 != '\0' {
              i += 1;
              temp.push(c1);
            }
            if c2 != c1 {
              ret.push_str(&temp);
              i += 1;
              continue 'a;
            }
          }
          ret.push_str(&targets[index]);
        }
      } else {
        ret.push(c1);
      }
      i += 1;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    s: String,
    indices: Vec<i32>,
    sources: Vec<String>,
    targets: Vec<String>,
    ret: String,
  }

  #[test]
  fn test_find_replace_string_simple() {
    let suites = vec![
      Suite {
        s: "abcd".to_string(),
        indices: vec![0, 2],
        sources: t1!["a", "cd"],
        targets: t1!["eee", "ffff"],
        ret: "eeebffff".to_string(),
      },
      Suite {
        s: "abcd".to_string(),
        indices: vec![0, 2],
        sources: t1!["ab", "ec"],
        targets: t1!["eee", "ffff"],
        ret: "eeecd".to_string(),
      },
    ];
    for s in suites {
      assert_eq!(
        s.ret,
        Solution::find_replace_string(s.s, s.indices, s.sources, s.targets)
      );
    }
  }
}
