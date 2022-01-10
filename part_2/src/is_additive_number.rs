use super::*;


fn dfs(digits: &Vec<u32>, temp: &mut Vec<u32>, prev_i: usize, i: usize, count: i32) -> bool {
  if digits.len() < temp.len() + i {
    return false;
  }
  if count < 2 {
    for j in i..digits.len() {
      if (j != i && digits[i] == 0) || (j - i > digits.len() - j - 1 || temp.len() > digits.len() - j - 1) {
        break;
      }
      add(temp, digits, i, j);
      if dfs(digits, temp, i, j + 1, count + 1) {
        return true;
      }
      if count == 0 {
        temp.drain(..);
      } else {
        if temp.len() > i - prev_i {
          temp.drain(i - prev_i..);
        }
        for j in 0..temp.len() {
          temp[j] = digits[prev_i + temp.len() - j - 1];
        }
      }
    }
  } else {
    let mut is_sum = true;
    for j in 0..temp.len() {
      if temp[j] != digits[i + temp.len() - j - 1] {
        is_sum = false;
        break;
      }
    }
    if is_sum {
      if i + temp.len() == digits.len() {
        return true;
      }
      let n = temp.len();
      add(temp, digits, prev_i, i - 1);
      if dfs(digits, temp, i, i + n, count + 1) {
        return true;
      }
      if temp.len() > n {
        temp.drain(n..);
      }
      for j in 0..n {
        temp[j] = digits[i + n - j - 1];
      }
    }
  }
  false
}

fn add(temp: &mut Vec<u32>, digits: &Vec<u32>, i: usize, j: usize) {
  let mut index = 0;
  let mut plus_one = false;
  while index < temp.len() || index < j + 1 - i || plus_one {
    let l = if index < temp.len() { temp[index] } else { 0 };
    let r = if index < j + 1 - i { digits[j - index] } else { 0 };
    let sum = l + r + if plus_one { 1 } else { 0 };
    plus_one = sum > 9;
    if index < temp.len() {
      temp[index] = sum % 10;
    } else {
      temp.push(sum % 10);
    }
    index += 1;
  }
}

impl Solution {
  pub fn is_additive_number(num: String) -> bool {
    dfs(&num.chars().map(|c| c.to_digit(10).unwrap()).collect(), &mut vec![], 0, 0, 0)
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    num: &'a str,
    ret: bool
  }

  #[test]
  fn test_is_additive_number_simple() {
    let suites = vec![
      Suite {
        num: "112358",
        ret: true
      },
      Suite {
        num: "199100199",
        ret: true
      },
      Suite {
        num: "101",
        ret: true
      },
      Suite {
        num: "211738",
        ret: true
      },
      Suite {
        num: "199111992",
        ret: true
      },
      Suite {
        num: "111122335588143",
        ret: true,
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::is_additive_number(s.num.to_string()));
    }
  }
}