use super::*;

impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        let mut counter = vec![0; 31];
        let mut factors = vec![0; 31];
        let mut options = vec![];
        for &num in &nums {
            if counter[num as usize] == 0 {
                let factor = Solution::prime_factor(num);
                if num == 1 || factor != 0 {
                    factors[num as usize] = factor;
                    options.push(num);
                }
            }
            counter[num as usize] += 1;
        }
        counter[1] = Solution::all_combos(counter[1] as i64);
        Solution::number_of_good_subsets_dfs(&options, &counter, &factors, 1, 0, 0) as i32
    }

    fn all_combos(num: i64) -> i32 {
      let mut ret = 1;
      for _ in 0..num {
        ret = (ret * 2) % 1000000007;
      }
      ret = if ret > 0 { ret - 1 } else { 1000000006 };
      ret
    }

    fn number_of_good_subsets_dfs(
        options: &Vec<i32>,
        counter: &Vec<i32>,
        factors: &Vec<i32>,
        count: i32,
        mask: i32,
        i: usize,
    ) -> i64 {
        let mut ret = 0;
        for j in i..options.len() {
            let option = options[j];
            if factors[option as usize] & mask == 0 {
                let mask = factors[option as usize] | mask;
                let count = (counter[option as usize] as i64 * count as i64) % 1000000007;
                ret = ((ret + if mask != 0 { count } else { 0 }) % 1000000007
                    + Solution::number_of_good_subsets_dfs(
                        options,
                        counter,
                        factors,
                        count as i32,
                        mask,
                        j + 1,
                    ))
                    % 1000000007;
            }
        }
        ret
    }

    fn prime_factor(mut num: i32) -> i32 {
        let mut ret = 0;
        while num != 1 {
            for i in 2..num + 1 {
                if num % i == 0 {
                    if (ret & (1 << i)) != 0 {
                        return 0;
                    } else {
                        ret |= 1 << i;
                    }
                    num /= i;
                    break;
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
        nums: Vec<i32>,
        ret: i32,
    }

    #[test]
    fn test_number_of_good_subsets_simple() {
        let suites = vec![
            Suite {
                nums: vec![1, 2, 3, 4],
                ret: 6,
            },
            Suite {
                nums: vec![4, 2, 3, 15],
                ret: 5,
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::number_of_good_subsets(s.nums));
        }
    }
}
