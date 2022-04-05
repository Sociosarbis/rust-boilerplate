use super::*;


static PRIMES: [bool;20] = [false, false, true, true, false, true, false, true, false, false, false, true, false, true, false, false, false, true, false, true];

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let mut ret = 0;
        for mut i in left..=right {
            let mut count = 0;
            while i != 0 {
                i &= i - 1;
                count += 1;
            }
            if PRIMES[count  as usize] {
                ret += 1;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        left: i32,
        right: i32,
        ret: i32
    }

    #[test]
    fn test_count_prime_set_bits_simple() {
        let suites = vec![
            Suite {
                left: 6,
                right: 10,
                ret: 4
            },
            Suite {
                left: 10,
                right: 15,
                ret: 5
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::count_prime_set_bits(s.left, s.right));
        }
    }
}