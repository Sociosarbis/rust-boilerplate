use super::*;

use std::cmp::Ordering;

impl Solution {
  pub fn is_rectangle_cover(mut rectangles: Vec<Vec<i32>>) -> bool {
    let mut x_min = rectangles[0][0];
    let mut y_min = rectangles[0][1];
    let mut x_max = rectangles[0][2];
    let mut y_max = rectangles[0][3];

    for i in 1..rectangles.len() {
      if x_min > rectangles[i][0] {
        x_min = rectangles[i][0];
      }
      if y_min > rectangles[i][1] {
        y_min = rectangles[i][1];
      }
      if x_max < rectangles[i][2] {
        x_max = rectangles[i][2];
      }
      if y_max < rectangles[i][3] {
        y_max = rectangles[i][3];
      }
    }
    rectangles.sort_by(|a, b| match a[1].cmp(&b[1]) {
      Ordering::Equal => a[0].cmp(&b[0]),
      v => v
    });
    let mut x_queue: Vec<usize> = vec![];
    let mut y_queue: Vec<usize> = vec![];
    let mut y = y_min;
    let mut width: i64 = 0;
    let mut i = 0;
    while i < rectangles.len() || !x_queue.is_empty() {
      while i < rectangles.len() && rectangles[i][1] <= y {
        println!("{:?}", i);
        if Solution::insert_x_queue(&mut x_queue, &rectangles, i) {
          Solution::insert_y_queue(&mut y_queue, &rectangles, i);
          width += rectangles[i][2] as i64 - rectangles[i][0] as i64;
        } else {
          return false;
        }
        i += 1;
      }
      if width != x_max as i64 - x_min as i64 {
        return false;
      }
      println!("{:?}", x_queue);
      y = rectangles[*y_queue.last().unwrap()][3];
      while !y_queue.is_empty() {
        let j = y_queue[y_queue.len() - 1];
        if rectangles[j][3] <= y {
          y_queue.pop();
          Solution::remove_x_queue(&mut x_queue, &rectangles, j);
          width -= rectangles[j][2] as i64 - rectangles[j][0] as i64;
        }
      }
    }
    y == y_max
  }

  fn remove_x_queue(x_queue: &mut Vec<usize>, rectangles: &Vec<Vec<i32>>, i: usize) {
    if !x_queue.is_empty() {
      let mut l = 0;
      let mut r = x_queue.len() - 1;
      while l <= r {
        let rect = &rectangles[i];
        let mid = (l + r) >> 1;
        let mid_rect = &rectangles[x_queue[mid]];
        if mid_rect[2] < rect[0] {
          l = mid + 1;
        } else if mid_rect[0] > rect[2] {
          if mid > 0 {
            r = mid - 1;
          } else {
            break;
          }
        } else {
          l = mid;
          break;
        }
      }
      if l < x_queue.len() {
        x_queue.remove(l);
      }
    }
  }

  fn insert_x_queue(x_queue:&mut Vec<usize>, rectangles: &Vec<Vec<i32>>, i: usize) -> bool {
    if x_queue.is_empty() {
      x_queue.push(i);
    } else {
      let rect = &rectangles[i];
      let mut l = 0;
      let mut r = x_queue.len() - 1;
      while l <= r {
        println!("{:?}", l);
        let mid = (l + r) >> 1;
        let mid_rect = &rectangles[x_queue[mid]];
        if mid_rect[2] < rect[0] {
          l = mid + 1;
        } else if mid_rect[0] > rect[2] {
          if mid > 0 {
            r = mid - 1;
          } else {
            break;
          }
        } else {
          if mid_rect[2] == rect[0] {
            l = mid + 1;
            break;
          } else if mid_rect[0] == rect[2] {
            l = mid;
            break;
          }
          return false;
        }
      }
      if l >= x_queue.len() {
        x_queue.push(i);
      } else {
        if rectangles[x_queue[l]][0] < rect[2] {
          return false
        }
        if l != 0 {
          if rectangles[x_queue[l - 1]][2] > rect[0] {
            return false
          }
        }
        x_queue.push(i);
      }
    }
    true
  }

  fn insert_y_queue(y_queue: &mut Vec<usize>, rectangles: &Vec<Vec<i32>>, i: usize) {
    if y_queue.is_empty() {
      y_queue.push(i);
    } else {
      let rect = &rectangles[i];
      let mut l = 0;
      let mut r = y_queue.len() - 1;
      while l <= r {
        let mid = (l + r) >> 1;
        let mid_rect = &rectangles[y_queue[mid]];
        if mid_rect[3] > rect[3] {
          l = mid + 1;
        } else if mid_rect[3] < rect[3] {
          if mid > 0 {
            r = mid - 1;
          } else {
            break;
          }
        } else {
          l = mid;
          break;
        }
      }
      if l >= y_queue.len() {
        y_queue.push(i);
      } else {
        y_queue.insert(l, i);
      }
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    rectangles: Vec<Vec<i32>>,
    ret: bool
  }

  #[test]
  fn test_is_rectangle_cover_simple() {
    let suites = vec![
      Suite {
        rectangles: t2_i![[1,1,3,3],[3,1,4,2],[3,2,4,4],[1,3,2,4],[2,3,3,4]],
        ret: true
      },
      Suite {
        rectangles: t2_i![[1,1,2,3],[1,3,2,4],[3,1,4,2],[3,2,4,4]],
        ret: false
      },
      Suite {
        rectangles: t2_i![[1,1,3,3],[3,1,4,2],[1,3,2,4],[3,2,4,4]],
        ret: false
      },
      Suite {
        rectangles: t2_i![[1,1,3,3],[3,1,4,2],[1,3,2,4],[2,2,4,4]],
        ret: false
      },
      Suite {
        rectangles: t2_i![[0,0,1,1],[0,1,3,2],[1,0,2,2]],
        ret: false
      },
      Suite {
        rectangles: t2_i![[0,0,3,3],[1,1,2,2]],
        ret: false
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::is_rectangle_cover(s.rectangles));
    }
  }
}