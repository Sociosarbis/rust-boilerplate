use super::*;

impl Solution {
  pub fn find_min_moves(machines: Vec<i32>) -> i32 {
    let sum = machines.iter().fold(0, |acc, num| acc + num);
    if sum % machines.len() as i32 != 0 {
      return -1;
    }
    let average = sum / machines.len() as i32;
    let mut ret = 0;
    let mut diff = 0;
    for i in 0..machines.len() {
      let v1 = machines[i] - average; 
      diff += v1;
      let v = if diff >= 0 { diff } else { -diff };
      if v > ret {
        ret = v;
      }
      if v1 > ret {
        ret = v1;
      }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    machines: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_find_min_moves_simple() {
    let suites = vec![
      Suite {
        machines: vec![1,0,5],
        ret: 3
      },
      Suite {
        machines: vec![0,3,0],
        ret: 2
      },
      Suite {
        machines: vec![0,2,0],
        ret: -1
      },
      Suite {
        machines: vec![4,0,0,4],
        ret: 2
      },
      Suite {
        machines: vec![4,4,0,0],
        ret: 4
      },
      Suite {
        machines: vec![100000,0,100000,0,100000,0,100000,0,100000,0,100000,0],
        ret: 50000
      },
      Suite {
        machines: vec![0,0,10,0,0,0,10,0,0,0],
        ret: 8
      },
      Suite {
        machines: vec![0,0,4,0,5,0,5,0,4],
        ret: 4
      },
      Suite {
        machines: vec![56,1,2,3,15,54,18,36,69,56],
        ret: 78
      },
      Suite {
        machines: vec![4,9,8,4,0],
        ret: 6
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_min_moves(s.machines));
    }
  }
}