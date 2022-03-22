use super::*;

impl Solution {
  pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut pop_points: Vec<(i32, i32)> = vec![];
    let mut stack: Vec<i32> = vec![];
    let mut ret:Vec<Vec<i32>> = vec![];
    for b in &buildings {
      pop_points.push((b[1], b[2]));
    }
    pop_points.sort_unstable_by(|a, b| {
      a.0.cmp(&b.0)
    });
    let mut i = 0;
    let mut j = 0;
    while i < buildings.len() || j < pop_points.len() {
      let left = if i < buildings.len() { buildings[i][0] } else { -1 };
      let right = pop_points[j].0;
      let mut cur = left;
      if left != -1 && left <= right {
        while i < buildings.len() && buildings[i][0] == left {
          let index = Solution::binary_search(&stack, buildings[i][2], true) as usize;
          if index < stack.len() {
            stack.insert(index, buildings[i][2]);
          } else {
            stack.push(buildings[i][2]);
          }
          i += 1;
        }
      }

      if left == -1 || right <= left {
        cur = right;
        while j < pop_points.len() && pop_points[j].0 == right {
          let index = Solution::binary_search(&stack, pop_points[j].1, false) as usize;
          stack.remove(index);
          j += 1;
        }
      }
      let next_num = if stack.is_empty() { 0 } else { stack[stack.len() - 1 ] };
      if !(!ret.is_empty() && ret[ret.len() - 1][1] == next_num) {
        ret.push(vec![cur, next_num]);
      }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    buildings: Vec<Vec<i32>>,
    ret: Vec<Vec<i32>>,
  }

  #[test]
  fn test_get_skyline_simple() {
    let suites = vec![
      Suite {
        buildings: t2_i![[2,9,10],[3,7,15],[5,12,12],[15,20,10],[19,24,8]],
        ret: t2_i![[2,10],[3,15],[7,12],[12,0],[15,10],[20,8],[24,0]]
      },
      Suite {
        buildings: t2_i![[0,2,3],[2,5,3]],
        ret: t2_i![[0,3],[5,0]]
      }
    ];
    
    for s in suites {
      assert_eq!(s.ret, Solution::get_skyline(s.buildings));
    }
  }
}