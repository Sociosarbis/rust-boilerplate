use super::*;

impl Solution {
  pub fn num_friend_requests(mut ages: Vec<i32>) -> i32 {
    ages.sort_unstable();
    let mut ret = 0;
    for i in 0..ages.len() {
      let age = ages[i];
      let target = age / 2 + 7;
      let mut left = Solution::binary_search(&ages, target, true);
      while (left as usize + 1) < ages.len() && ages[left as usize + 1] == target {
        left +=1;
      }
      while (left as usize) < ages.len() && left >= 0 && ages[left as usize] > target {
        left -= 1;
      }
      if ((left + 1) as usize) < ages.len() {
        let mut index = Solution::binary_search(&ages, age, true);
        while index < ages.len() as i32 && ages[index as usize] <= age {
          index += 1;
        }
        if index > left + 1 {
          ret += index - (left + 1);
          if (i as i32) < index && i as i32 > left {
            ret -= 1;
          }
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
    ages: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_num_friend_requests_simple() {
    let suites = vec![
      Suite {
        ages: vec![16,16],
        ret: 2
      },
      Suite {
        ages: vec![16,17,18],
        ret: 2
      },
      Suite {
        ages: vec![20,30,100,110,120],
        ret: 3
      },
      Suite {
        ages: vec![101,98,80,20,1,97,3,77,114,109],
        ret: 21
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::num_friend_requests(s.ages));
    }
  }
}