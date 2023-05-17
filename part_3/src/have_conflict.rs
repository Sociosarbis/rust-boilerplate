use super::*;

impl Solution {
  pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
    let r1: Vec<u16> = event1
      .into_iter()
      .map(|t| {
        t.split(':')
          .map(|d| d.parse::<u16>().unwrap())
          .enumerate()
          .fold(0, |acc, (i, d)| acc + if i == 0 { d * 60 } else { d })
      })
      .collect();

    let r2: Vec<u16> = event2
      .into_iter()
      .map(|t| {
        t.split(':')
          .map(|d| d.parse::<u16>().unwrap())
          .enumerate()
          .fold(0, |acc, (i, d)| acc + if i == 0 { d * 60 } else { d })
      })
      .collect();

    !(r1[1] < r2[0] || r1[0] > r2[1])
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    event1: Vec<String>,
    event2: Vec<String>,
    ret: bool,
  }

  #[test]
  fn test_have_conflict_simple() {
    let suites = vec![
      Suite {
        event1: t1!["01:15", "02:00"],
        event2: t1!["02:00", "03:00"],
        ret: true,
      },
      Suite {
        event1: t1!["01:00", "02:00"],
        event2: t1!["01:20", "03:00"],
        ret: true,
      },
      Suite {
        event1: t1!["10:00", "11:00"],
        event2: t1!["14:00", "15:00"],
        ret: false,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::have_conflict(s.event1, s.event2));
    }
  }
}
