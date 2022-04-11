use super::*;

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut ret = 0;
        for i in 1..=n {
            let mut temp = 10;
            let mut base = 1;
            for j in 1..=i {
                base *= if j == 1 && i != 1 { 9 } else { temp };
                temp -= 1;
            }
            ret += base;
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        n: i32,
        ret: i32
    }

    #[test]
    fn test_count_numbers_with_unique_digits_simple() {
        let suites = vec![
            Suite {
                n: 2,
                ret: 91
            },
            Suite {
                n: 0,
                ret: 1
            }
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::count_numbers_with_unique_digits(s.n));
        }
    }
}