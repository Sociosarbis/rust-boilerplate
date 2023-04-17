use super::*;

static MONTHS: [i32;13] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];

impl Solution {
  pub fn count_days_together(arrive_alice: String, leave_alice: String, arrive_bob: String, leave_bob: String) -> i32 {
    let start_a = Solution::to_days(&arrive_alice);
    let end_a = Solution::to_days(&leave_alice);
    let start_b = Solution::to_days(&arrive_bob);
    let end_b = Solution::to_days(&leave_bob);

    let start = if start_a > start_b { start_a } else { start_b };
    let end = if end_a < end_b { end_a } else { end_b };

    if start > end {
      0
    } else {
      end - start + 1
    }
  }

  fn to_days(s: &String) -> i32 {
    s.split('-').enumerate().map(|(i, s)| {
      let num = s.parse::<i32>().unwrap();
      if i == 0 {
        MONTHS[num as usize - 1]
      } else {
        num
      }
    }).fold(0, |acc, item| acc + item)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    arrive_alice: String,
    leave_alice: String,
    arrive_bob: String,
    leave_bob: String,
    ret: i32
  }

  #[test]
  fn test_count_days_together_simple() {
    let suites = vec![
      Suite {
        arrive_alice: "08-15".to_string(),
        leave_alice: "08-18".to_string(),
        arrive_bob: "08-16".to_string(),
        leave_bob: "08-19".to_string(),
        ret: 3
      },
      Suite {
        arrive_alice: "10-01".to_string(),
        leave_alice: "10-31".to_string(),
        arrive_bob: "11-01".to_string(),
        leave_bob: "12-31".to_string(),
        ret: 0
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_days_together(s.arrive_alice, s.leave_alice, s.arrive_bob, s.leave_bob));
    }
  }
}