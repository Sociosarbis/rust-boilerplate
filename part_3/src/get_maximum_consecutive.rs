use super::*;

impl Solution {
    pub fn get_maximum_consecutive(mut coins: Vec<i32>) -> i32 {
      coins.sort();
      let mut ret = 0;
      for coin in coins {
        if coin > ret + 1 {
          break
        }
        ret += coin;
      }
      return ret + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        coins: Vec<i32>,
        ret: i32,
    }

    #[test]
    fn test_get_maximum_consecutive_simple() {
        let suites = vec![
            Suite {
                coins: vec![1, 3],
                ret: 2,
            },
            Suite {
                coins: vec![1,1,1,4],
                ret: 8,
            },
            Suite {
                coins: vec![1,4,10,3,1],
                ret: 20,
            },
        ];

        for s in suites {
          assert_eq!(s.ret, Solution::get_maximum_consecutive(s.coins));
        }
    }
}
