use super::*;

impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let mut prev = (0, 0);
        let mut ret: Vec<char> = vec![];
        for c in target.chars() {
            let (x, y) = ((c as i16 - 97) % 5, (c as i16 - 97) / 5);
            let v = y - prev.1;
            let h = x - prev.0;
            if c == 'z' {
                for _ in 0..(-h) {
                    ret.push('L');
                }
                for _ in 0..v {
                    ret.push('D');
                }
                ret.push('!');
            } else {
                if v > 0 {
                    for _ in 0..v {
                        ret.push('D');
                    }
                } else if v < 0 {
                    for _ in 0..(-v) {
                        ret.push('U');
                    }
                }

                if h > 0 {
                    for _ in 0..h {
                        ret.push('R');
                    }
                } else if h < 0 {
                    for _ in 0..(-h) {
                        ret.push('L');
                    }
                }
                ret.push('!');
            }
            prev = (x, y)
        }
        ret.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        target: String,
        ret: String,
    }

    #[test]
    fn test_alphabet_board_path_simple() {
        let suites = vec![
            Suite {
                target: "leet".to_string(),
                ret: "DDR!UURRR!!DDD!".to_string(),
            },
            Suite {
                target: "code".to_string(),
                ret: "RR!DDRR!UUL!R!".to_string(),
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::alphabet_board_path(s.target));
        }
    }
}
