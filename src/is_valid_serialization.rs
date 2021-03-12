use super::Solution;

impl Solution {
  pub fn is_valid_serialization(preorder: String) -> bool {
      let nodes: Vec<&str> = preorder.split(',').collect();
      let mut stack = vec![];
      let mut i = 0;
      while i < nodes.len() {
        if !stack.is_empty() {
          stack.pop();
        } else if i != 0 {
          return false;
        }
        while i < nodes.len() && nodes[i] != '#'.to_string() {
          stack.push(nodes[i]);
          i += 1;
        }
        i += 1;
      }
      stack.is_empty()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    preorder: String,
    ret: bool
  }

  #[test]
  fn is_valid_serialization_simple() {
    let suites = vec![
      Suite {
        preorder: "9,3,4,#,#,1,#,#,2,#,6,#,#".to_string(),
        ret: true
      },
      Suite {
        preorder: "1,#".to_string(),
        ret: false,
      },
      Suite {
        preorder: "9,#,#,1".to_string(),
        ret: false
      },
      Suite {
        preorder: "1,#,#,#,#".to_string(),
        ret: false
      },
      Suite {
        preorder: "#".to_string(),
        ret: true
      },
      Suite {
        preorder: "9,#,92,#,#".to_string(),
        ret: true
      }
    ];

    for s in suites {
      assert_eq!(Solution::is_valid_serialization(s.preorder), s.ret);
    }
  }
}