use super::*;

impl Solution {
  pub fn min_operations_max_profit(customers: Vec<i32>, boarding_cost: i32, running_cost: i32) -> i32 {
    let mut max = 0;
    let mut customer_count = 0;
    let mut ret = 0;
    let mut i = 0;
    let mut temp = 0;
    while i < customers.len() || customer_count != 0 {
      temp -= running_cost;
      if i < customers.len() {
        customer_count += customers[i];
      }
      if customer_count >= 4 {
        temp += 4 * boarding_cost;
        customer_count -= 4;
      } else {
        temp += customer_count * boarding_cost;
        customer_count = 0;
      }
      if temp > max {
        max = temp;
        ret = i as i32 + 1;
      }
      i += 1;
    }
    if max > 0 {
      ret
    } else {
      -1
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    customers: Vec<i32>,
    boarding_cost: i32,
    running_cost: i32,
    ret: i32
  }

  #[test]
  fn test_min_operations_max_profit_simple() {
    let suites = vec![
      Suite {
        customers: vec![8,3],
        boarding_cost: 5,
        running_cost: 6,
        ret: 3
      },
      Suite {
        customers: vec![10,9,6],
        boarding_cost: 6,
        running_cost: 4,
        ret: 7
      },
      Suite {
        customers: vec![3,4,0,5,1],
        boarding_cost: 1,
        running_cost: 92,
        ret: -1
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_operations_max_profit(s.customers, s.boarding_cost, s.running_cost));
    }
  }
}