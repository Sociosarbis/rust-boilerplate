use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let mut map = HashMap::new();
    for i in  0..list1.len() {
      map.insert(list1[i].clone(), i);
    }

    let mut min = list1.len() + list2.len();
    let mut ret = vec![];
    for i in 0..list2.len() {
      if let Some(&j) = map.get(&list2[i]) {
        if i + j <= min {
          if i + j < min {
            ret.clear();
            min = i + j;
          }
          ret.push(list2[i].clone());
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
    list1: Vec<String>,
    list2: Vec<String>,
    ret: Vec<String>
  }

  #[test]
  fn test_find_restaurant_simple() {
    let suites = vec![
      Suite {
        list1: t1!["Shogun","Tapioca Express","Burger King","KFC"],
        list2: t1!["Piatti","The Grill at Torrey Pines","Hungry Hunter Steakhouse","Shogun"],
        ret: t1!["Shogun"]
      },
      Suite {
        list1: t1!["Shogun","Tapioca Express","Burger King","KFC"],
        list2: t1!["KFC","Shogun","Burger King"],
        ret: t1!["Shogun"]
      }
    ];

    for s in suites {
      assert_eq!(s.ret,  Solution::find_restaurant(s.list1, s.list2));
    }
  }
}