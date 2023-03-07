use super::*;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let (mut res, _) = Solution::brace_expansion_ii_dfs(&expression.chars().collect(), 0);
        res.sort();
        let mut ret = vec![];
        for (i, item) in res.iter().enumerate() {
            if i + 1 < res.len() && *item == res[i + 1] {
                continue;
            }
            ret.push(item.clone());
        }
        ret
    }

    fn brace_expansion_ii_dfs(expression: &Vec<char>, mut i: usize) -> (Vec<String>, usize) {
        let is_start = i == 0;
        let mut ret = vec![];
        if i < expression.len() {
            let mut temp = String::new();
            let mut left: Vec<String> = vec![];
            while i < expression.len() {
                match expression[i] {
                    '{' => {
                        let (res, next_i) = Solution::brace_expansion_ii_dfs(expression, i + 1);
                        i = next_i;
                        if left.is_empty() {
                            if temp.is_empty() {
                                left.extend(res);
                            } else {
                                for b in &res {
                                    left.push(format!("{}{}", temp, b));
                                }
                            }
                        } else {
                            let mut next_left = vec![];
                            for a in &left {
                                for b in &res {
                                    next_left.push(format!("{}{}{}", a, temp, b));
                                }
                            }
                            left = next_left;
                        }
                        temp.clear();
                    }
                    c @ (',' | '}') => {
                        if !temp.is_empty() {
                          if !left.is_empty() {
                            for a in &left {
                              ret.push(format!("{}{}", a, temp));
                            }
                            left.clear();
                          } else {
                            ret.push(temp.clone());
                          }
                          temp.clear();
                        }
                        i += 1;
                        if c != ',' {
                            if !(is_start && i < expression.len()) {
                                if !left.is_empty() {
                                    ret.extend(left.clone());
                                    left.clear();
                                }
                                break;
                            }
                        } else {
                            if !left.is_empty() {
                                ret.extend(left.clone());
                                left.clear();
                            }
                        }
                    }
                    c => {
                        temp.push(c);
                        i += 1;
                    }
                }
            }
            if !left.is_empty() {
              for a in &left {
                ret.push(format!("{}{}", a, temp));
              }
            } else if !temp.is_empty() {
              ret.push(temp);
            }
        }
        (ret, i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        expression: String,
        ret: Vec<String>,
    }

    #[test]
    fn test_brace_expansion_ii_simple() {
        let suites = vec![
            Suite {
                expression: "{a,b}{c,{d,e}}".to_string(),
                ret: t1!["ac", "ad", "ae", "bc", "bd", "be"],
            },
            Suite {
                expression: "{{a,z},a{b,c},{ab,z}}".to_string(),
                ret: t1!["a", "ab", "ac", "z"],
            },
            Suite {
                expression: "{a,b}c{d,e}f".to_string(),
                ret: t1!["acdf", "acef", "bcdf", "bcef"],
            },
            Suite {
              expression: "{a{x,ia,o}w,{n,{g,{u,o}},{a,{x,ia,o},w}},er}".to_string(),
              ret: t1!["a","aiaw","aow","axw","er","g","ia","n","o","u","w","x"]
            }
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::brace_expansion_ii(s.expression));
        }
    }
}
