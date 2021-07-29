use super::Solution;


impl Solution {
  pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
    let mut max_per_layer = vec![1];
    while max_per_layer[max_per_layer.len() - 1] < label {
      max_per_layer.push(2 * max_per_layer[max_per_layer.len() - 1] + 1);
    }
    let mut cur = label;
    let mut ret = vec![cur];
    while ret.len() < max_per_layer.len() {
      let i = max_per_layer.len() - ret.len();
      let is_reversed = (i & 1) == 1;
      let mut num = cur;
      if is_reversed {
        num = max_per_layer[i - 1] + max_per_layer[i] - cur + 1
      }
      cur = (if (num & 1) == 1 { num - 1 } else { num }) / 2;
      if !is_reversed && i > 1 {
        cur = max_per_layer[i - 2] + max_per_layer[i - 1] - cur + 1;
      }
      ret.push(cur);
    }
    ret.reverse();
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    label: i32,
    ret: Vec<i32>
  }

  #[test]
  fn test_path_in_zig_zag_tree_simple() {
    let suites = vec![
      Suite {
        label: 14,
        ret: vec![1,3,4,14]
      },
      Suite {
        label: 26,
        ret: vec![1,2,6,10,26]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::path_in_zig_zag_tree(s.label));
    }
  }
}