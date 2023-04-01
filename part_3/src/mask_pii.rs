use super::*;

impl Solution {
    pub fn mask_pii(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        match chars[0] {
            first_ch @ ('a'..='z' | 'A'..='Z') => {
                let mut ret = String::new();
                if first_ch >= 'A' && first_ch <= 'Z' {
                    ret.push((first_ch as u8 + 32) as char);
                } else {
                    ret.push(first_ch);
                }
                let mut i = 1;
                while i < chars.len() {
                    if chars[i + 1] == '@' {
                        ret.push_str(&"*".repeat(5));
                        if chars[i] >= 'A' && chars[i] <= 'Z' {
                            ret.push((chars[i] as u8 + 32) as char);
                        } else {
                            ret.push(chars[i]);
                        }
                        i += 2;
                        ret.push('@');
                        while i < chars.len() {
                            if chars[i] >= 'A' && chars[i] <= 'Z' {
                                ret.push((chars[i] as u8 + 32) as char);
                            } else {
                                ret.push(chars[i]);
                            }
                            i += 1;
                        }
                        break;
                    }
                    i += 1;
                }
                ret
            }
            _ => {
              let mut digits: Vec<char> = vec![];
              for i in 0..chars.len() {
                if chars[i] >= '0' && chars[i] <= '9' {
                  digits.push(chars[i]);
                }
              }
              let mut ret = String::new();
              if digits.len() > 10 {
                ret.push('+');
                ret.push_str(&"*".repeat(digits.len() - 10));
                ret.push('-');
              }
              ret.push_str("***-***-");
              for i in digits.len() - 4..digits.len() {
                ret.push(digits[i]);
              }
              ret
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        s: String,
        ret: String,
    }

    #[test]
    fn test_mask_pii_simple() {
        let suites = vec![
            Suite {
                s: "LeetCode@LeetCode.com".to_string(),
                ret: "l*****e@leetcode.com".to_string(),
            },
            Suite {
                s: "AB@qq.com".to_string(),
                ret: "a*****b@qq.com".to_string(),
            },
            Suite {
                s: "1(234)567-890".to_string(),
                ret: "***-***-7890".to_string(),
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::mask_pii(s.s));
        }
    }
}
