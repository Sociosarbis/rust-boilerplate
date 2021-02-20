use super::Solution;

impl Solution {
  pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut num_map: Vec<(i32, i32, i32)> = vec![(-1, -1, 0);50000];
    let mut degree_i = nums[0] as usize;
    num_map[degree_i].0 = 0;
    num_map[degree_i].1 = 1;
    num_map[degree_i].2 = 1;
    for i in 1..nums.len() {
      let num = nums[i] as usize;
      if num_map[num].0 == -1 {
        num_map[num].0 = i as i32;
        num_map[num].1 = 1;
        num_map[num].2 = 1;
      } else {
        num_map[num].1 = (i + 1) as i32 - num_map[num].0;
        num_map[num].2 += 1;
      }
      if num != degree_i {
        if num_map[num].2 > num_map[degree_i].2 || (num_map[num].2 == num_map[degree_i].2 && num_map[num].1 < num_map[degree_i].1) {
          degree_i = num;
        }
      }
    }
    return num_map[degree_i].1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn find_shortest_sub_array_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,2,2,3,1],
        ret: 2
      },
      Suite {
        nums: vec![1,2,2,3,1,4,2],
        ret: 6,
      }
    ];

    for su in suites {
      assert_eq!(Solution::find_shortest_sub_array(su.nums), su.ret);
    }
  }
}