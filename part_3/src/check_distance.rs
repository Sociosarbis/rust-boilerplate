use super::*;

impl Solution {
  pub fn check_distances(s: String, mut distance: Vec<i32>) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for (i, &ch) in chars.iter().enumerate() {
      let index = ch as usize - 97;
      if distance[index] >= 0 {
        let next_index = i + distance[index] as usize + 1;
        if next_index < chars.len() && chars[next_index] == ch {
          distance[index] = -1;
        } else {
          return false;
        }
      }
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    s: String,
    distance: Vec<i32>,
    ret: bool
  }

  #[test]
  fn test_check_distance_simple() {
    let suites = vec![
      Suite {
        s: "abaccb".to_string(),
        distance: vec![1,3,0,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        ret: true,
      },
      Suite {
        s: "aa".to_string(),
        distance: vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        ret: false,
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::check_distances(s.s, s.distance));
    }
  }
}