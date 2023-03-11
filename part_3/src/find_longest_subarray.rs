use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
    let mut m: HashMap<i32, usize> = HashMap::new();
    let mut a = 0;
    let mut n = 0;
    let mut ret = (0, 0);
    for (i, item) in array.iter().enumerate() {
      for ch in item.chars() {
        if ch >= '0' && ch <= '9' {
          n += 1;
        } else {
          a += 1;
        }
        break;
      }
      if a == n {
        ret = (0, i  + 1)
      } else {
        let k = a - n;
        if let Some(&j) = m.get(&k) {
          if i - j > ret.1 - ret.0 {
            ret = (j + 1, i + 1);
          }
        }
        if !m.contains_key(&k) {
          m.insert(k, i);
        }
      }
    }
    array.into_iter().skip(ret.0).take(ret.1 - ret.0).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    array: Vec<String>,
    ret: Vec<String>
  }

  #[test]
  fn test_find_longest_subarray_simple() {
    let suites = vec![
      Suite {
        array: t1!["A","1","B","C","D","2","3","4","E","5","F","G","6","7","H","I","J","K","L","M"],
        ret: t1!["A","1","B","C","D","2","3","4","E","5","F","G","6","7"]
      },
      Suite {
        array: t1!["A","A"],
        ret: t1![]
      },
      Suite {
        array: t1!["42","10","O","t","y","p","g","B","96","H","5","v","P","52","25","96","b","L","Y","z","d","52","3","v","71","J","A","0","v","51","E","k","H","96","21","W","59","I","V","s","59","w","X","33","29","H","32","51","f","i","58","56","66","90","F","10","93","53","85","28","78","d","67","81","T","K","S","l","L","Z","j","5","R","b","44","R","h","B","30","63","z","75","60","m","61","a","5","S","Z","D","2","A","W","k","84","44","96","96","y","M"],
        ret: t1!["52","3","v","71","J","A","0","v","51","E","k","H","96","21","W","59","I","V","s","59","w","X","33","29","H","32","51","f","i","58","56","66","90","F","10","93","53","85","28","78","d","67","81","T","K","S","l","L","Z","j","5","R","b","44","R","h","B","30","63","z","75","60","m","61","a","5"]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_longest_subarray(s.array));
    }
  }
}