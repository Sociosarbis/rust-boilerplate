use super::*;


impl Solution {
  pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
      return 0;
    }
    nums.sort();
    let mut ret = 0;
    for i in 0..nums.len() - 2 {
      let a = nums[i];
      let mut l = i + 2;
      for j in i + 1..nums.len() - 1 {
        let b = a + nums[j];
        if l < j + 1 {
          l = j + 1;
        }
        l = Solution::binary_search_general(&nums, b, l, nums.len() - 1, true) as usize + 1;
        while l > j {
          if l - 1 < nums.len() && nums[l - 1] < b {
            l -= 1;
            ret += l - j;
            break;
          }
          l -= 1;
        }
      }
    }
    ret as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_triangle_number_simple() {
    let suites = vec![
      /*Suite {
        nums: vec![2,2,3,4],
        ret: 3
      },*/
      Suite {
        nums: vec![4,2,3,4],
        ret: 4
      }
    ];
    for s in suites {
      assert_eq!(s.ret, Solution::triangle_number(s.nums));
    }
  }
}