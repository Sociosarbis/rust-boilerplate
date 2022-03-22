use super::*;

impl Solution {
    pub fn winner_of_game(mut colors: String) -> bool {
        let mut a_moves = 0;
        let mut b_moves = 0;

        colors.push(' ');
        let mut temp = 0;
        let mut prev = colors.chars().next().unwrap();
        for c in colors.chars() {
            if c == prev {
                temp += 1;
            } else {
                match prev {
                    'A' => {
                        if temp > 2 {
                            a_moves += temp - 2;
                        }
                    },
                    'B' => {
                        if temp > 2 {
                            b_moves += temp - 2;
                        }
                    },
                    _ => {}
                }
                temp = 1;
                prev = c;
            }
        }
        b_moves < a_moves

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct Suite<'a> {
        colors: &'a str,
        ret: bool
    }

    #[test]
    fn test_winner_of_game_simple() {
        let suites = vec![
            Suite {
                colors: "AAABABB",
                ret: true
            },
            Suite {
                colors: "AA",
                ret: false
            },
            Suite {
                colors: "ABBBBBBBAAA",
                ret: false
            }
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::winner_of_game(s.colors.to_string()));
        }
    }
}