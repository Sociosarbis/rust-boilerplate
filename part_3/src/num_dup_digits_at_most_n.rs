use super::*;

impl Solution {
    pub fn num_dup_digits_at_most_n(mut n: i32) -> i32 {
        if n < 11 {
            return 0;
        }
        let mut nums = vec![];
        while n != 0 {
            nums.push(n % 10);
            n /= 10;
        }
        let mut ret = 0;
        let mut total = 1;
        let mut not_dup = 0;
        for i in 1..nums.len() {
            let mut temp = if i == 1 { 1 } else { 9 };
            for j in if i == 1 { 0 } else { 1 }..i {
                temp *= 10 - j as i32;
            }
            total *= 10;
            not_dup += temp;
        }
        ret += total - not_dup;
        let mut visited = vec![false; 10];
        for i in (0..nums.len()).rev() {
            for j in if i + 1 == nums.len() { 1 } else { 0 }..nums[i] as usize {
                if !visited[j] {
                    let mut temp = 1;
                    for k in 0..i {
                        temp *= ((10 - (nums.len() - i)) - k) as i32;
                    }
                    ret += total - temp;
                } else {
                  ret += total;
                }
            }
            if visited[nums[i] as usize] {
                total /= 10;
                for j in (0..i).rev() {
                    ret += nums[j] * total;
                    total /= 10;
                }
                ret += 1;
                break;
            }
            visited[nums[i] as usize] = true;
            total /= 10;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        n: i32,
        ret: i32,
    }

    #[test]
    fn test_num_dup_digits_at_most_n_simple() {
        let suites = vec![
            Suite { n: 20, ret: 1 },
            Suite { n: 100, ret: 10 },
            Suite { n: 1000, ret: 262 },
            Suite { n: 110, ret: 12 },
            Suite { n: 111, ret: 13 }
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::num_dup_digits_at_most_n(s.n));
        }
    }
}
