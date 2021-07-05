use super::Solution;

use std::collections::HashMap;


impl Solution {
  pub fn count_of_atoms(formula: String) -> String {
    let res = Solution::count_of_atoms_dfs(&formula.chars().collect(), 0);
    let mut atoms: Vec<&String> = res.1.keys().collect();
    atoms.sort();
    let mut ret = String::new();
    for atom in atoms {
      ret.push_str(&atom.as_str());
      let num = res.1.get(atom).unwrap();
      if *num > 1 {
        ret.push_str(num.to_string().as_str());
      }
    }
    ret
  }

  fn count_of_atoms_dfs(chars: &Vec<char>, mut index: usize) -> (usize, HashMap<String, i32>) {
    let mut from = -1;
    let mut temp_str = String::new();
    let mut digit_from = -1;
    let mut ret: HashMap<String, i32> = HashMap::new();
    let mut temp_group: Option<HashMap<String, i32>> = None;
    while index <= chars.len() {
      let c = {
        if index == chars.len() {
          '\0'
        } else {
          chars[index]
        }
      };
      if (c >= 'A' && c <= 'Z') || index == chars.len() || c == ')' || c == '(' {
        let num: i32 = {
          if digit_from != -1 {
            let num_str: String = chars[digit_from as usize..index].iter().collect();
            num_str.parse::<i32>().unwrap()
          } else {
            1
          }
        };
  
        if temp_group.is_some() {
          let group = temp_group.unwrap();
          for (k, v) in group.iter() {
            if !ret.contains_key(k) {
              ret.insert(k.to_owned(), 0);
            }
            *ret.get_mut(k).unwrap() += num * v;
          }
          temp_group = None;
        } else {
          if from != -1 {
            temp_str = chars[from as usize..index].iter().collect();
            from = -1;
          }
          if !temp_str.is_empty() {
            if !ret.contains_key(&temp_str) {
              ret.insert(temp_str.to_owned(), 0);
            }
            *ret.get_mut(&temp_str).unwrap() += num;
            temp_str.clear();
          }
        }
        if c == ')' {
          break;
        }

        if c == '(' {
          let res = Solution::count_of_atoms_dfs(chars, index + 1);
          index = res.0;
          temp_group = Some(res.1);
        } else {
          from = index as i32;
        }
        digit_from = -1;
      } else if c >= '0' && c <= '9' {
        if digit_from == -1 {
          if from != -1 {
            temp_str = chars[from as usize..index].iter().collect();
            from = -1;
          }
          digit_from = index as i32;
        }
      }
      index += 1;
    }
    (index, ret)
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    formula: &'a str,
    ret: &'a str
  }

  #[test]
  fn test_count_of_atoms_simple() {
    let suites = vec![
      Suite {
        formula: "H2O",
        ret: "H2O"
      },
      Suite {
        formula: "Mg(OH)2",
        ret: "H2MgO2"
      },
      Suite {
        formula: "K4(ON(SO3)2)2",
        ret: "K4N2O14S4"
      },
      Suite {
        formula: "Be32",
        ret: "Be32"
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_of_atoms(s.formula.to_owned()));
    }
  }
}