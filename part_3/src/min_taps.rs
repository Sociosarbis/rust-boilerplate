use super::*;

impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        let mut max = 0;
        for (i, &range) in ranges.iter().enumerate() {
            if range != 0 {
                let i = i as i32;
                let l = if i > range { i - range } else { 0 };
                let r = if i + range < n { i + range } else { n };
                if l == 0 {
                    for j in l..=r {
                        dp[j as usize] = 1;
                    }
                    max = r;
                } else if r > max {
                    let mut min = n + 1;
                    for j in l..=max {
                        if dp[j as usize] != 0 && dp[j as usize] < min {
                            min = dp[j as usize];
                        }
                    }
                    if min != n + 1 {
                        for j in max + 1..=r {
                            if dp[j as usize] == 0 || dp[j as usize] > min + 1 {
                                dp[j as usize] = min + 1;
                            }
                        }
                        max = r;
                    }
                }
            }
        }
        for &v in &dp {
            if v == 0 {
                return -1;
            }
        }
        dp[dp.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        n: i32,
        ranges: Vec<i32>,
        ret: i32,
    }

    #[test]
    fn test_min_taps_simple() {
        let suites = vec![
            Suite {
                n: 5,
                ranges: vec![3, 4, 1, 1, 0, 0],
                ret: 1,
            },
            Suite {
                n: 3,
                ranges: vec![0, 0, 0, 0],
                ret: -1,
            },
            Suite {
                n: 9,
                ranges: vec![0, 5, 0, 3, 3, 3, 1, 4, 0, 4],
                ret: 2,
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::min_taps(s.n, s.ranges));
        }
    }
}
