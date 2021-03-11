use super::Solution;

impl Solution {
  pub fn calculate2(mut s: String) -> i32 {
    let mut stack: Vec<String> = vec![];
    s = format!("({})", s);
    let chars: Vec<char> = s.chars().collect();
    let mut index = 0;
    let mut eager_eval = false;
    while index < chars.len() {
      let c = chars[index];
      match c {
        ' ' => (),
        ')' => {
          let mut i = stack.len() - 1;
          while stack[i] != '('.to_string() {
            i -= 1;
          }
          let mut j = i + 1;
          if j < stack.len() {
            let mut res = {
              let r = stack[j].parse::<i32>();
              if r.is_ok() {
                r.unwrap()
              } else {
                0
              }
            };

            j += 1;

            while j < stack.len() {
              let r = stack[j].parse::<i32>();
              if r.is_ok() {
                let num = r.unwrap();
                if stack[j - 1] == '+'.to_string() {
                  res += num;
                } else if stack[j - 1] == '-'.to_string() {
                  res -= num;
                }
              }
              j += 1;
            }
    
            if index == chars.len() - 1 {
                return res;
            } else {
                stack.resize(i, "\0".to_string());
                stack.push(res.to_string());
                eager_eval = true;
            }
          }
        },
        _ => {
          if c.is_digit(10) {
            let mut num_str = String::from(c);
            while index + 1 < chars.len() && chars[index + 1].is_digit(10) {
              num_str.push(chars[index + 1]);
              index += 1;
            }
            stack.push(num_str);
            eager_eval = true;
          } else {
            stack.push(c.to_string());
          }
        }
      }

      if eager_eval {
        if stack.len() > 2 {
          if stack[stack.len() - 2] == "*".to_string() || stack[stack.len() - 2] == "/".to_string() {
            let right = stack[stack.len() - 1].parse::<i32>().unwrap();
            let left = stack[stack.len() - 3].parse::<i32>().unwrap();
            let res = {
              if stack[stack.len() - 2] == "*".to_string() {
                left * right
              } else {
                left / right
              }
            };
            stack.resize(stack.len() - 3, "\0".to_string());
            stack.push(res.to_string());
          }
        }
        eager_eval = false;
      }
      index += 1;
    }
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    s: String,
    ret: i32
  }

  #[test]
  fn calculate_simple() {
    let suites = vec![
      Suite {
        s: "3+2*2".to_string(),
        ret: 7
      },
      Suite {
        s: " 3/2 ".to_string(),
        ret: 1
      },
      Suite {
        s: " 3+5 / 2 ".to_string(),
        ret: 5
      }
    ];

    for s in suites {
      assert_eq!(Solution::calculate2(s.s), s.ret);
    }
  }
}