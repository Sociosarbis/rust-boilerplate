use super::*;

impl Solution {
  pub fn query_string(s: String, n: i32) -> bool {
    let mut count = 0;
    let mut temp = n as usize;
    while temp != 0 {
      temp >>= 1;
      count += 1;
    }
    if count > s.len() {
      return false;
    }
    let bits: Vec<u8> = s.bytes().map(|b| b - 48).collect();
    let mut visited = vec![false; n as usize + 1];
    visited[0] = true;
    for i in 1..=count {
      temp = 0;
      let mask = (1 << i) - 1;
      for (j, &bit) in bits.iter().enumerate().take(i) {
        temp |= (bit as usize) << (i - j - 1);
      }
      if temp > 0 && temp < visited.len() && !visited[temp] {
        visited[temp] = true;
      }
      for j in 1..=bits.len() - i {
        temp = (temp << 1) & mask;
        temp |= bits[j + i - 1] as usize;
        if temp > 0 && temp < visited.len() && !visited[temp] {
          visited[temp] = true;
        }
      }
    }
    visited.into_iter().all(|s| s)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    s: String,
    n: i32,
    ret: bool,
  }

  #[test]
  fn test_query_string_simple() {
    let suites = vec![
      Suite {
        s: "0110".to_string(),
        n: 3,
        ret: true,
      },
      Suite {
        s: "0110".to_string(),
        n: 4,
        ret: false,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::query_string(s.s, s.n));
    }
  }
}
