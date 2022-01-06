use super::*;

impl Solution {
  pub fn simplify_path(path: String) -> String {
    let segments = path.split("/");
    let mut ret = vec![];
    for segment in segments {
      match segment {
        "" | "." => {},
        ".." => {
          if !ret.is_empty() {
            ret.pop();
          }
        },
        s => {
          ret.push(s);
        }
      }
    }
    format!("/{}", ret.join("/"))
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    path: &'a str,
    ret: &'a str
  }

  #[test]
  fn test_simplify_path_simple() {
    let suites = vec![
      Suite {
        path: "/home/",
        ret: "/home"
      },
      Suite {
        path: "/../",
        ret: "/"
      },
      Suite {
        path: "/home//foo/",
        ret: "/home/foo"
      }
    ];

    for s in suites {
      assert_eq!(s.ret.to_string(), Solution::simplify_path(s.path.to_string()));
    }
  }
}