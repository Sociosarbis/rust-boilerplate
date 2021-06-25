use super::Solution;

impl Solution {
  pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let target_num = target.parse::<usize>().unwrap();
    let mut deadend_table = [false;10000];
    let mut visited = [false;10000];
    let mut queue = vec![0];
    let mut round = 0;
    for d in deadends {
      deadend_table[d.parse::<usize>().unwrap()] = true;
    }
    if !deadend_table[0] {
      visited[0] = true;

      while !queue.is_empty() {
        let n = queue.len();
        for i in 0..n {
          let num = queue[i];
          if num == target_num {
            return round;
          } else {
            let d1 = num % 10;
            let d2 = (num / 10) % 10;
            let d3 = (num / 100) % 10;
            let d4 = (num / 1000) % 10;
            
            let mut new_nums = vec![];
            {
              let base = d2 * 10 + d3 * 100 + d4 * 1000;
              let new_num1 = base + if d1 == 0 { 9 } else { d1 - 1 };
              let new_num2 = base + if d1 == 9 { 0 } else { d1 + 1 };
              new_nums.push(new_num1);
              new_nums.push(new_num2);
            }
            {
              let base = d1 + d3 * 100 + d4 * 1000;
              let new_num1 = base + (if d2 == 0 { 9 } else { d2 - 1 }) * 10;
              let new_num2 = base + (if d2 == 9 { 0 } else { d2 + 1 }) * 10;
              new_nums.push(new_num1);
              new_nums.push(new_num2);
            }
            {
              let base = d1 + d2 * 10 + d4 * 1000;
              let new_num1 = base + (if d3 == 0 { 9 } else { d3 - 1 }) * 100;
              let new_num2 = base + (if d3 == 9 { 0 } else { d3 + 1 }) * 100;
              new_nums.push(new_num1);
              new_nums.push(new_num2);
            }
            {
              let base = d1 + d2 * 10 + d3 * 100;
              let new_num1 = base + (if d4 == 0 { 9 } else { d4 - 1 }) * 1000;
              let new_num2 = base + (if d4 == 9 { 0 } else { d4 + 1 }) * 1000;
              new_nums.push(new_num1);
              new_nums.push(new_num2);
            }
            for new_num in new_nums {
              if !deadend_table[new_num] && !visited[new_num] {
                visited[new_num] = true;
                queue.push(new_num);
              }
            }
          }
        }
        queue.drain(0..n);
        round += 1;
      }
    }
    
    return -1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    deadends: Vec<&'a str>,
    target: &'a str,
    ret: i32
  }

  #[test]
  fn test_open_lock_simple() {
    let suites = vec![
      Suite {
        deadends: vec!["0201","0101","0102","1212","2002"],
        target: "0202",
        ret: 6
      },
      Suite {
        deadends: vec!["8888"],
        target: "0009",
        ret: 1
      },
      Suite {
        deadends: vec!["8887","8889","8878","8898","8788","8988","7888","9888"],
        target: "8888",
        ret: -1
      },
      Suite {
        deadends: vec!["0000"],
        target: "8888",
        ret: -1
      }
    ];

    for s in suites {
      assert_eq!(Solution::open_lock(Solution::t1(s.deadends), s.target.to_owned()), s.ret);
    }
  }
}