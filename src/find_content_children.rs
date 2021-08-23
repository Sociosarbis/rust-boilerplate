use super::Solution;

impl Solution {
  pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
      let mut g_mut = g;
      let mut s_mut = s;
      g_mut.sort_unstable();
      s_mut.sort_unstable();
      let mut j: usize = 0;
      for i in 0..g_mut.len() {
        if j == s_mut.len() { return i as i32; }
        while j < s_mut.len() {
          if s_mut[j] < g_mut[i] {
            j += 1;
            if j == s_mut.len() { return i as i32; }
          } else {
            j += 1;
            break;
          }
        }
      }
      g_mut.len() as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    g: Vec<i32>,
    s: Vec<i32>,
    ret: i32
  }

  #[test]
  fn find_content_children_simple() {
    let suites: Vec<Suite> = vec![
      Suite {
        g: vec![1,2,3],
        s: vec![1,1],
        ret: 1
      },
      Suite {
        g: vec![1,2],
        s: vec![1,2,3],
        ret: 2
      }
    ];
    for s in suites {
      assert_eq!(Solution::find_content_children(s.g, s.s), s.ret);
    }
  }
}

