use super::*;

impl Solution {
  pub fn push_dominoes(dominoes: String) -> String {
    let mut dominoes: Vec<char> = dominoes.chars().collect();
    let mut counter = vec![0;dominoes.len()];
    for i in 0..dominoes.len() {
      match dominoes[i] {
        'R' => {
          counter[i] = 1;
          for j in i + 1..dominoes.len() {
            if dominoes[j] == '.' {
              counter[j] = counter[j - 1] + 1;
            } else {
              break;
            }
          }
        },
        'L' => {
          counter[i] = -1;
          for j in (0..i).rev() {
            if counter[j] == 0 {
              counter[j] = counter[j + 1] - 1;
            } else if counter[j] > 0 {
              let res = counter[j] + counter[j + 1] - 1;
              if res > 0 {
                counter[j] = counter[j + 1] - 1;
              } else {
                if res == 0 {
                  counter[j] = 0;
                }
                break;
              }
            } else {
              break;
            }
          }
        },
        _ => {}
      }
    }
    for i in 0..counter.len() {
      dominoes[i] = if counter[i] > 0 { 'R' } else if counter[i] < 0 { 'L' } else { '.' };
    }
    dominoes.into_iter().collect()
  }

  pub fn push_dominoes_best(dominoes: String) -> String {
    let mut dominoes: Vec<char> = dominoes.chars().collect();
    let mut count = 0;
    for i in 0..dominoes.len() {
      match dominoes[i] {
        'R' => {
          if count > 1 {
            for j in i - count + 1..i {
              dominoes[j] = 'R';
            }
          }
          count = 1;
        },
        'L' => {
          if count == 0 {
            for j in (0..i).rev() {
              if dominoes[j] == '.' {
                dominoes[j] = 'L'
              } else {
                  break;
              }
            }
          } else {
            let half_count = (count - 1) >> 1;
            for j in i - half_count..i {
              dominoes[j] = 'L';
            }
            for j in i - count + 1..i - count + 1 + half_count {
              dominoes[j] = 'R'
            }
            count = 0;
          }
        },
        _ => {
          if count > 0 {
            count += 1;
          }
        }
      }
    }
    if count > 1 {
      for j in dominoes.len() - count + 1..dominoes.len() {
        dominoes[j] = 'R';
      }
    }
    dominoes.into_iter().collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    dominoes: &'a str,
    ret: &'a str
  }

  #[test]
  fn test_push_dominoes_simple() {
    let suites = vec![
      Suite {
        dominoes: "RR.L",
        ret: "RR.L"
      },
      Suite {
        dominoes: ".L.R...LR..L..",
        ret: "LL.RR.LLRRLL.."
      }
    ];

    for s in suites {
      assert_eq!(s.ret.to_string(), Solution::push_dominoes(s.dominoes.to_string()));
    }
  }
}