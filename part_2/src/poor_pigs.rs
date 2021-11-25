use super::*;

impl Solution {
  pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
    if buckets == 1 {
      return 0;
    }
    // 猪的数量表示空间的维数，可测试次数 + 1表示，各维可取值的范围
    // 每次测试，猪会去喝到沿着各个维方向上下个超平面上的所有桶水，
    // 整个问题可以理解为一个限制了大小的空间中寻找一个点，其为各个维中死于毒药的交集
    // 整个空间包含的点即为可确认的桶数
    let turns = minutes_to_test / minutes_to_die;
    // 测试次数 + 1是因为，当测试次数都用完的时候，只会剩下一个桶
    // 其是否为毒药是可以确定的
    let base = turns + 1;
    let mut res = base;
    let mut ret = 1;
    while res < buckets {
      res *= base;
      ret += 1;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    buckets: i32,
    minutes_to_die: i32,
    minutes_to_test: i32,
    ret: i32
  }

  #[test]
  fn test_poor_pigs_simple() {
    let suites = vec![
      Suite {
        buckets: 1000,
        minutes_to_die: 15,
        minutes_to_test: 60,
        ret: 5
      },
      Suite {
        buckets: 4,
        minutes_to_die: 15,
        minutes_to_test: 15,
        ret: 2
      },
      Suite {
        buckets: 4,
        minutes_to_die: 15,
        minutes_to_test: 30,
        ret: 2
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::poor_pigs(s.buckets, s.minutes_to_die, s.minutes_to_test));
    }
  }
}