use super::*;

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut ret = vec![0;n as usize];
        let mut stack: Vec<(usize, i32)> = vec![];
        for log in &logs {
            let mut sp = log.split(':');
            let i = sp.next().unwrap().parse::<usize>().unwrap();
            let t = sp.next();
            let c = sp.next().unwrap().parse::<i32>().unwrap();
            match t {
                Some("start") => {
                    stack.push((i, c));
                }
                Some("end") => {
                    let top = stack.pop().unwrap();
                    let time = c - top.1 + 1;
                    if !stack.is_empty() {
                        let last = stack.last().unwrap();
                        if last.0 != i {
                            ret[last.0] -= time;
                        } else {
                            continue;
                        }
                    }
                    ret[i] += time;
                }
                _ => unreachable!()
            }
        }
        ret
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        n: i32,
        logs: Vec<String>,
        ret: Vec<i32>
    }

    #[test]
    fn test_exclusive_time_simple() {
        let suites = vec![
            Suite {
                n: 2,
                logs: t1!["0:start:0","1:start:2","1:end:5","0:end:6"],
                ret: vec![3,4]
            },
            Suite {
                n: 1,
                logs: t1!["0:start:0","0:start:2","0:end:5","0:start:6","0:end:6","0:end:7"],
                ret: vec![8],
            },
            Suite {
                n: 2,
                logs: t1!["0:start:0","0:start:2","0:end:5","1:start:6","1:end:6","0:end:7"],
                ret: vec![7,1],
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::exclusive_time(s.n, s.logs));
        }
    }
}