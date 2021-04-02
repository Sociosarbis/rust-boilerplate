use super::Solution;

impl Solution {
  pub fn trap(mut height: Vec<i32>) -> i32 {
    let mut dir = 0;
    let mut start = 0;
    
    let mut ret = 0;
    height.push(0);
    for i in 1..height.len() {
      if height[i] - height[i - 1] < 0 {
        if dir == 1 {
          let top = if height[i - 1] < height[start] { height[i - 1] } else { height[start] };
          for j in start + 1..i-1 {
            if top > height[j] {
              ret += top - height[j];
              height[j] = top;
            }
          }
          if height[i - 1] >= height[start] {
            start = i - 1;
          }
        }
        if dir != -1 {
          dir = -1;
        }
      } else if height[i] - height[i - 1] > 0 {
        if dir != 1 {
          dir = 1;
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
    height: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_trap_simple() {
    let suites = vec![
      Suite {
        height: vec![0,1,0,2,1,0,1,3,2,1,2,1],
        ret: 6
      },
      Suite {
        height: vec![5,4,1,2],
        ret: 1
      }
    ];
    for s in suites {
      assert_eq!(Solution::trap(s.height), s.ret);
    }
  }
}