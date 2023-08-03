use super::*;

#[derive(Debug, Copy, Clone)]
enum State {
  Initial,
  ReadBlock,
}

impl Solution {
  pub fn remove_comments(source: Vec<String>) -> Vec<String> {
    let mut state = State::Initial;
    let mut temp = String::new();
    let mut ret = vec![];
    for line in source {
      let mut read_chars = String::with_capacity(line.len());
      let mut chars = line.chars();
       while let Some(c) = chars.next() {
        match state {
          State::Initial => {
            if c == '/' {
              if let Some(next) = chars.next() {
                match next {
                  '/' => {
                    break;
                  }
                  '*' => {
                    state = State::ReadBlock;
                  }
                  _ => {
                    read_chars.push(c);
                    read_chars.push(next);
                  }
                }
                continue;
              }
            }
            read_chars.push(c);
          }
          _ => {
            if c == '*' {
              while let Some(next) = chars.next() {
                if next == '*' {
                  continue;
                }
                if next == '/' {
                  state = State::Initial;
                }
                break;
              }
            }
          }
        }
      }
      match state {
        State::ReadBlock => {
          temp.push_str(&read_chars);
        }
        _ => {
          if !temp.is_empty() {
            temp.push_str(&read_chars);
            ret.push(temp);
            temp = String::new();
          } else {
            if !read_chars.is_empty() {
              ret.push(read_chars);
            }
          }
        }
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    source: Vec<String>,
    ret: Vec<String>,
  }

  #[test]
  fn test_remove_comments_simple() {
    let suites = vec![
      Suite {
        source: t1![
          "/*Test program */",
          "int main()",
          "{ ",
          "  // variable declaration ",
          "int a, b, c;",
          "/* This is a test",
          "   multiline  ",
          "   comment for ",
          "   testing */",
          "a = b + c;",
          "}"
        ],
        ret: t1!["int main()", "{ ", "  ", "int a, b, c;", "a = b + c;", "}"],
      },
      Suite {
        source: t1!["a/*comment", "line", "more_comment*/b"],
        ret: t1!["ab"],
      },
      Suite {
        source: t1![
          "struct Node{",
          "    /*/ declare members;/**/",
          "    int size;",
          "    /**/int val;",
          "};"
        ],
        ret: t1![
          "struct Node{",
          "    ",
          "    int size;",
          "    int val;",
          "};"
        ],
      },
      Suite {
        source: t1![
          "void func(int k) {",
          "// this function does nothing /*",
          "   k = k*2/4;",
          "   k = k/2;*/",
          "}"
        ],
        ret: t1!["void func(int k) {", "   k = k*2/4;", "   k = k/2;*/", "}"],
      },
      Suite {
        source: t1!["a/*/b//*c","blank","d/*/e*//f"],
        ret: t1!["ae*"]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::remove_comments(s.source));
    }
  }
}
