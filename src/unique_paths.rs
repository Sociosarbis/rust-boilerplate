struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let t = m + n - 1;
        let mut s = m;
        let mut r = n;
        if n > m {
            s = n;
            r = m;
        }
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let tmp = b;
                b = a % b;
                a = tmp;
            }
            return a;
        }

        let mut ret = 1;
        let mut d = 1;
        for i in 2..r {
            d *= i;
        }
        for i in s..t {
            ret *= i;
            let g = gcd(ret, d);
            if g != 1 {
                ret /= g;
                d /= g;
            }
        }
        return (ret / d) as i32;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn unique_paths_simple() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(7, 3), 28);
    }
}
