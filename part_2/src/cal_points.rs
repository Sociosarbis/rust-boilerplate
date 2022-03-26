use super::*;

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut ret = 0;
        let mut temps: Vec<i32> = vec![];
        for op in &ops {
            match op.as_str() {
                "C" => {
                    temps.pop().unwrap();
                },
                "+" => {
                    let n = temps.len();
                    temps.push(temps[n - 1] + temps[n - 2]);
                },
                "D" => {
                    temps.push(temps.last().unwrap() * 2);
                },
                s => {
                    temps.push(s.parse::<i32>().unwrap());
                }
            }
        }
        temps.iter().sum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        ops: Vec<String>,
        ret: i32
    }

    #[test]
    fn test_cal_points_simple() {
        let suites = vec![
            Suite {
                ops: t1!["5","2","C","D","+"],
                ret: 30
            },
            Suite {
                ops: t1!["5","-2","4","C","D","9","+","+"],
                ret: 27
            },
            Suite {
                ops: t1!["1"],
                ret: 1
            }
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::cal_points(s.ops));
        }
    }
}