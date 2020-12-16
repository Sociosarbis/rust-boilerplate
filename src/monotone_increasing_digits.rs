struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut digits: Vec<i32> = vec![];
        let mut base = 1;
        let mut ret_digits: Vec<i32> = vec![];
        for i in 0..10 {
            if n >= base {
                while digits.len() < i {
                    digits.push(0);
                }
                digits.push((n / base) % 10);
                base *= 10;
            } else {
                break;
            }
        }

        ret_digits.push(digits[digits.len() - 1]);
        for i in (0..digits.len() - 1).rev() {
            let mut j = ret_digits.len() - 1;
            if digits[i] < ret_digits[j] {
                ret_digits[j] -= 1;
                while j > 0 && ret_digits[j - 1] > ret_digits[j] {
                    ret_digits[j - 1] -= 1;
                    j -= 1;
                }
                for k in j + 1..ret_digits.len() {
                    ret_digits[k] = 9;
                }
                while ret_digits.len() < digits.len() {
                    ret_digits.push(9);
                }
                break;
            } else {
                ret_digits.push(digits[i]);
            }
        }
        let mut ret = 0;
        base = 1;
        for i in (0..ret_digits.len()).rev() {
            ret += base * ret_digits[i];
            base *= 10;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monotone_increasing_digits_simple() {
        assert_eq!(Solution::monotone_increasing_digits(232), 229);
        assert_eq!(Solution::monotone_increasing_digits(10), 9);
        assert_eq!(Solution::monotone_increasing_digits(1234), 1234);
    }
}
