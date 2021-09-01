use super::*;


impl Solution {
  pub fn compare_version(version1: String, version2: String) -> i32 {
    let v1: Vec<&str> = version1.split(".").collect();
    let v2: Vec<&str> = version2.split(".").collect();
    let mut i = 0;
    let mut j = 0;
    while i < v1.len() || j < v2.len() {
      let l = {
        if i < v1.len() {
          let old_i = i;
          i += 1;
          v1[old_i].parse::<i32>().unwrap()
        } else {
          0
        }
      };

      let r = {
        if j < v2.len() {
          let old_j = j;
          j += 1;
          v2[old_j].parse::<i32>().unwrap()
        } else {
          0
        }
      };
      if l < r {
        return -1;
      } else if l > r {
        return 1;
      }
    }
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    version1: &'a str,
    version2: &'a str,
    ret: i32
  }

  #[test]
  fn test_compare_version_simple() {
    let suites = vec![
      Suite {
        version1: "1.01",
        version2: "1.001",
        ret: 0
      },
      Suite {
        version1: "1.0",
        version2: "1.0.0",
        ret: 0
      },
      Suite {
        version1: "0.1",
        version2: "1.1",
        ret: -1
      },
      Suite {
        version1: "1.0.1",
        version2: "1",
        ret: 1
      },
      Suite {
        version1: "7.5.2.4",
        version2: "7.5.3",
        ret: -1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::compare_version(s.version1.to_owned(), s.version2.to_owned()));
    }
  }
}