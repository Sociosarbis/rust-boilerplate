use super::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Token {
    Num(i32, Box<str>),
    Op(char),
}

fn dfs(tokens: &mut Vec<Token>, dp: &mut HashSet<String>, ret: &mut Vec<i32>) {
    for i in (0..tokens.len() - 2).step_by(2) {
        if let Token::Num(left, left_expr) = tokens[i].clone() {
            if let Token::Op(op) = tokens[i + 1] {
                if let Token::Num(right, right_expr) = tokens[i + 2].clone() {
                    let res = match op {
                        '+' => left + right,
                        '-' => left - right,
                        '*' => left * right,
                        _ => unreachable!(),
                    };
                    if tokens.len() == 3 {
                        let expr = format!("({}{}{})", left_expr, op, right_expr);
                        if !dp.contains(&expr) {
                            dp.insert(expr);
                            ret.push(res);
                        }
                    } else {
                        tokens.splice(
                            i..=i + 2,
                            [Token::Num(
                                res,
                                format!("({}{}{})", left_expr, op, right_expr).into(),
                            )],
                        );
                        dfs(tokens, dp, ret);
                        tokens.splice(
                            i..=i,
                            [
                                Token::Num(left, left_expr),
                                Token::Op(op),
                                Token::Num(right, right_expr),
                            ],
                        );
                    }
                }
            }
        }
    }
}

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut items = vec![];
        let mut temp = String::new();
        for c in expression.chars() {
            match c {
                '+' | '-' | '*' => {
                    items.push(Token::Num(temp.parse::<i32>().unwrap(), temp.clone().into()));
                    temp.clear();
                    items.push(Token::Op(c));
                }
                _ => {
                    temp.push(c);
                }
            }
        }
        if !temp.is_empty() {
            items.push(Token::Num(temp.parse::<i32>().unwrap(), temp.clone().into()));
        }
        let mut ret = vec![];
        let mut dp = HashSet::new();
        dfs(&mut items, &mut dp, &mut ret);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite<'a> {
        expression: &'a str,
        ret: Vec<i32>,
    }

    #[test]
    fn test_diff_ways_to_compute() {
        let suites = vec![
            Suite {
                expression: "2-1-1",
                ret: vec![0, 2],
            },
            Suite {
                expression: "2*3-4*5",
                ret: vec![-34, -14, -10, -10, 10],
            },
        ];

        for s in suites {
            assert_eq!(
                s.ret,
                Solution::diff_ways_to_compute(s.expression.to_string())
            );
        }
    }
}
