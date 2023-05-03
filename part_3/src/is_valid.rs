use super::*;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.bytes() {
            match c {
              b'c' => {
                if stack.len() >= 2
                    && stack[stack.len() - 1] == b'b'
                {
                    stack.drain(stack.len() - 2..);
                    continue;
                } else {
                  return false;
                }
              }
              b'b' => {
                if stack.is_empty() || stack[stack.len() - 1] != b'a' {
                  return false;
                }
              }
              _ => {}
            }
            stack.push(c);
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        s: String,
        ret: bool,
    }

    #[test]
    fn test_is_valid_simple() {
        let suites = vec![
            Suite {
                s: "aabcbc".to_string(),
                ret: true,
            },
            Suite {
                s: "abcabcababcc".to_string(),
                ret: true,
            },
            Suite {
                s: "abccba".to_string(),
                ret: false,
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::is_valid(s.s));
        }
    }
}
