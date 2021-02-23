use super::Solution;

impl Solution {
  pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
      let grumpy_minutes = grumpy.iter().enumerate().fold(vec![], |mut acc, (i, &num)| {
        if num == 1 {
          acc.push(i);
        }
        return acc;
      });
      let mut j = -1;
      let mut ret = 0;
      let mut tmp = 0;
      for i in 0..grumpy_minutes.len() {
        let index = grumpy_minutes[i];
        if j == -1 {
          j = i as i32;
          ret = customers[index];
          tmp = ret;
        } else {
          while (j as usize) < grumpy_minutes.len() && index as i32 - grumpy_minutes[j as usize] as i32  >= x {
            tmp -= customers[grumpy_minutes[j as usize]];
            j +=1;
          }
          tmp += customers[index];
          if tmp > ret {
            ret = tmp;
          }
        }
      }

      customers.iter().enumerate().fold(0, |mut acc, (i, &num)| {
        if grumpy[i] == 0 {
          acc += num;
        }
        return acc;
      }) + ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    customers: Vec<i32>,
    grumpy: Vec<i32>,
    x: i32,
    ret: i32,
  }

  #[test]
  fn max_satisfied_simple() {
    let suites = vec![
      Suite {
        customers: vec![1,0,1,2,1,1,7,5],
        grumpy: vec![0,1,0,1,0,1,0,1],
        x: 3,
        ret: 16
      }
    ];

    for s in suites {
      assert_eq!(Solution::max_satisfied(s.customers, s.grumpy, s.x), s.ret);
    }
  }
}