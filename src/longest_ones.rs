use super::Solution;

impl Solution {
  pub fn longest_ones(a: Vec<i32>, k: i32) -> i32 {
    let mut range: Vec<(i32, i32)> = vec![];
    let mut i = 0;
    for j in 0..a.len() {
      if a[j] != 1 {
        if j > i {
          range.push((i as i32, (j - i) as i32));
        }
        i = j + 1;
      }
    }
    if a.len() > i {
      range.push((i as i32, (a.len() - i) as i32));
    }
    i = 0;
    let mut tmp = if !range.is_empty() { range[0].1 } else { 0 };
    let mut ret = tmp + k;
    let mut tmp_k = k;
    for j in 1..range.len() {
      let diff = range[j].0 - range[j - 1].0 - range[j - 1].1;
      if diff > k {
        tmp_k = k;
        tmp = 0;
        i = j;
      } else {
        while i < j && range[j].0 - range[j - 1].0 - range[j - 1].1 > tmp_k {
          tmp_k += range[i + 1].0 - range[i].0 - range[i].1;
          tmp -= range[i].1;
          i += 1;
        }
        tmp_k -= diff;
      }
      tmp += range[j].1;
      if tmp + k > ret {
        ret = tmp + k;
      }
    }
    return if ret > a.len() as i32 { a.len() as i32 } else { ret };
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    a: Vec<i32>,
    k: i32,
    ret: i32
  }

  #[test]
  fn longest_ones_simple() {
    let suites = vec![
      Suite {
        a: vec![1,1,1,0,0,0,1,1,1,1,0],
        k: 2,
        ret: 6
      },
      Suite {
        a: vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1],
        k: 3,
        ret: 10
      }
    ];

    for su in suites {
      assert_eq!(Solution::longest_ones(su.a, su.k), su.ret);
    }
  }
}