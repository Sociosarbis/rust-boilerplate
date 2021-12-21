use super::*;

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let nums: Vec<i32> = date
            .split("-")
            .into_iter()
            .map(|item| item.parse::<i32>().unwrap())
            .collect();
        (1..nums[1]).fold(0, |acc, m| {
            acc + (if m < 8 {
                if m == 2 {
                    if (nums[0] & 3) == 0 {
                        29
                    } else {
                        28
                    }
                } else if (m & 1) == 1 {
                    31
                } else {
                    30
                }
            } else {
                if (m & 1) == 1 {
                    30
                } else {
                    31
                }
            })
        }) + nums[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite<'a> {
        date: &'a str,
        ret: i32,
    }

    #[test]
    fn test_day_of_year_simple() {
        let suites = vec![
            Suite {
                date: "2019-01-09",
                ret: 9,
            },
            Suite {
                date: "2019-02-10",
                ret: 41,
            },
            Suite {
                date: "2003-03-01",
                ret: 60,
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::day_of_year(s.date.to_owned()));
        }
    }
}
