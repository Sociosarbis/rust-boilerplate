use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
    let mut m: HashMap<String, i32> = HashMap::new();
    let mut ret = vec![];
    for name in &names {
      if let Some(&v) = m.get(name) {
        let mut next = v + 1;
        while m.contains_key(&format!("{}({})", name, next)) {
          next += 1;
        }
        m.insert(name.clone(), next);
        let s = format!("{}({})", name, next);
        m.insert(s.clone(), 0);
        ret.push(s);
      } else {
        m.insert(name.clone(), 0);
        ret.push(name.clone());
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    names: Vec<String>,
    ret: Vec<String>
  }

  #[test]
  fn test_get_folder_names_simple() {
    let suites = vec![
      Suite {
        names: t1!["pes","fifa","gta","pes(2019)"],
        ret: t1!["pes","fifa","gta","pes(2019)"]
      },
      Suite {
        names: t1!["gta","gta(1)","gta","avalon"],
        ret: t1!["gta","gta(1)","gta(2)","avalon"]
      },
      Suite {
        names: t1!["kaido","kaido(1)","kaido","kaido(1)","kaido(2)"],
        ret: t1!["kaido","kaido(1)","kaido(2)","kaido(1)(1)","kaido(2)(1)"]
      },
      Suite {
        names: t1!["kingston(0)","kingston","kingston"],
        ret: t1!["kingston(0)","kingston","kingston(1)"]
      },
      Suite {
        names: t1!["b(4)(3)","d(1)","k","z(1)(4)","u","s(1)(2)","q(1)(4)","z(1)","r","b(3)(4)","x","e","r(1)","t","e","z(2)","d","n(1)","o","o","t(1)","l","p","a","w","j(3)","w","c(3)","q","v","u"],
        ret: t1!["b(4)(3)","d(1)","k","z(1)(4)","u","s(1)(2)","q(1)(4)","z(1)","r","b(3)(4)","x","e","r(1)","t","e(1)","z(2)","d","n(1)","o","o(1)","t(1)","l","p","a","w","j(3)","w(1)","c(3)","q","v","u(1)"]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::get_folder_names(s.names));
    }
  }
}