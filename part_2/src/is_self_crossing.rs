use super::*;


impl Solution {
  pub fn is_self_crossing(mut distance: Vec<i32>) -> bool {
    if distance.len() < 4 {
      return false;
    }
    let mut i = 2;
    // 因为路径是螺旋状的，只有当螺旋扩张，或者缩小时，才不会相交
    // 先遍历扩张的路径
    while i < distance.len() && distance[i] > distance[i - 2] {
      i += 1;
    }
    if i == distance.len() {
      return false
    }
    // 当路径从扩张转向缩小时，需要对上一步的长度进行特殊处理
    // 为两个螺旋相互独立时，上一步长度的最大值
    if (i == 3 && distance[i] == distance[i - 2]) || (i >= 4 && distance[i] >= distance[i - 2] - distance[i - 4]) {
      distance[i - 1] -= distance[i - 3]
    }
    i += 1;
    while i < distance.len() && distance[i] < distance[i - 2] {
      i += 1;
    }
    i != distance.len()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    distance: Vec<i32>,
    ret: bool
  }

  #[test]
  fn test_is_self_crossing_simple() {
    let suites = vec![
      Suite {
        distance: vec![2,1,1,2],
        ret: true
      },
      Suite {
        distance: vec![1,2,3,4],
        ret: false
      },
      Suite {
        distance: vec![1,1,1,1],
        ret: true
      },
      Suite {
        distance: vec![2,2,3,2,1],
        ret: true
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::is_self_crossing(s.distance));
    }
  }
}