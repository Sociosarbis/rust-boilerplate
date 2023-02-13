use super::*;

impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let mut counter = [0; 4];
        let n = s.len() as i32;
        let avg = n / 4;
        let chars: Vec<char> = s.chars().collect();
        for &c in &chars {
            counter[Solution::to_index(c)] += 1;
        }
        let mut min = 0;
        let mut least = [0; 4];
        for i in 0..4 {
            if counter[i] > avg {
                least[i] = counter[i] - avg;
                min += least[i];
            }
            counter[i] = 0;
        }
        let mut ret = 0;
        let mut end = 0;
        for j in end..s.len() {
            let index = Solution::to_index(chars[j]);
            counter[index] += 1;
            if counter
                .iter()
                .enumerate()
                .find(|(i, &c)| c < least[*i])
                .is_none()
            {
                end = j + 1;
                ret = end as i32;
                break;
            }
        }

        if ret != min {
            'outer: for i in 1..=(s.len() - min as usize) {
                let index = Solution::to_index(chars[i - 1]);
                counter[index] -= 1;
                if counter[index] < least[index] {
                    for j in end..s.len() {
                        let index = Solution::to_index(chars[j]);
                        counter[index] += 1;
                        if counter[index] == least[index] {
                            end = j + 1;
                            let next = (end - i) as i32;
                            if next < ret {
                                ret = next;
                                if ret == min {
                                    break 'outer;
                                }
                            }
                            continue 'outer;
                        }
                    }
                    break 'outer;
                } else {
                    let next= (end - i) as i32;
                    if next < ret {
                        ret = next;
                        if ret == min {
                            break;
                        }
                    }
                }
            }
        }
        ret
    }

    fn to_index(c: char) -> usize {
        match c {
            'Q' => 0,
            'W' => 1,
            'E' => 2,
            _ => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        s: String,
        ret: i32,
    }

    #[test]
    fn test_balenced_string_simple() {
        let suites = vec![
            // Suite {
            //     s: "QWER".to_string(),
            //     ret: 0,
            // },
            // Suite {
            //     s: "QQWE".to_string(),
            //     ret: 1,
            // },
            // Suite {
            //     s: "QQQW".to_string(),
            //     ret: 2,
            // },
            // Suite {
            //     s: "WQWRQQQW".to_string(),
            //     ret: 3,
            // },
            Suite {
                s: "WWEQERQWQWWRWWERQWEQ".to_string(),
                ret: 4
            }
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::balanced_string(s.s));
        }
    }
}
