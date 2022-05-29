use super::*;

impl Solution {
    pub fn valid_ip_address(mut query_ip: String) -> String {
        let mut ret = "Neither";
        let mut count = 0;
        for c in query_ip.chars() {
            match c {
                '.' => {
                    ret = "IPv4";
                    break;
                }
                ':' => {
                    ret = "IPv6";
                    break;
                }
                _ => {}
            }
        }
        if ret != "Neither" {
            if ret == "IPv4" {
                query_ip.push('.');
            } else {
                query_ip.push(':');
            }
            let mut temp = String::new();
            for c in query_ip.chars() {
                if ret == "IPv4" {
                    match c {
                        '0'..='9' => {
                            temp.push(c);
                        }
                        '.' => {
                            if temp.len() > 0 && temp.len() <= 3 {
                                if !(temp.len() != 1 && temp.chars().next().unwrap() == '0') {
                                    if let Ok(res) = temp.parse::<i32>() {
                                        if res >= 0 && res <= 255 {
                                            count += 1;
                                            temp.clear();
                                            continue;
                                        }
                                    }
                                }
                            }
                            ret = "Neither";
                            break;
                        }
                        _ => {
                            ret = "Neither";
                            break;
                        }
                    }
                } else {
                    match c {
                        '0'..='9' | 'a'..='f' | 'A'..='F' => {
                            temp.push(c);
                        }
                        ':' => {
                            if temp.len() > 0 && temp.len() <= 4 {
                                count += 1;
                                temp.clear();
                                continue;
                            }
                            ret = "Neither";
                            break;
                        }
                        _ => {
                            ret = "Neither";
                            break;
                        }
                    }
                }
            }
        }

        match ret {
            "IPv4" => {
                if count != 4 {
                    ret = "Neither";
                } 
            }
            "IPv6" => {
                if count != 8 {
                    ret = "Neither";
                }
            }
            _ => {}
        }
        ret.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite<'a> {
        query_ip: &'a str,
        ret: &'a str,
    }

    #[test]
    fn test_valid_ip_address_simple() {
        let suites = vec![
            Suite {
                query_ip: "172.16.254.1",
                ret: "IPv4",
            },
            Suite {
                query_ip: "2001:0db8:85a3:0:0:8A2E:0370:7334",
                ret: "IPv6",
            },
            Suite {
                query_ip: "256.256.256.256",
                ret: "Neither",
            },
        ];

        for s in suites {
            assert_eq!(
                s.ret.to_string(),
                Solution::valid_ip_address(s.query_ip.to_string())
            );
        }
    }
}
