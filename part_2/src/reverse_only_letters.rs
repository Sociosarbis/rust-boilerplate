use super::*;

impl Solution {
  pub fn reverse_only_letters(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut j = s.len() - 1;
    loop {
      while i < s.len() {
        match chars[i] {
          'a'..='z' | 'A'..='Z' => {
            break;
          },
          _ => {
            i += 1;
          }
        }
      }

      loop {
        match chars[j] {
          'a'..='z' | 'A'..='Z' => {
            break;
          },
          _ => {
            if j > 0 {
              j -= 1;
            } else {
              break;
            }
          }
        }
      }
      if i < j {
        chars.swap(i, j);
        i += 1;
        if j > 0 {
          j -= 1;
        }
      } else {
        break;
      }
    }
    chars.into_iter().collect()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    s: &'a str,
    ret: &'a str
  }

  #[test]
  fn test_reverse_only_letters_simple() {
    let suites = vec![
      Suite {
        s: "ab-cd",
        ret: "dc-ba"
      },
      Suite {
        s: "ab-cd",
        ret: "dc-ba"
      },
      Suite {
        s: "Test1ng-Leet=code-Q!",
        ret: "Qedo1ct-eeLg=ntse-T!"
      }
    ];

    for s in suites {
      assert_eq!(s.ret.to_string(), Solution::reverse_only_letters(s.s.to_string()));
    }
  }
}