use super::*;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut timestamps: Vec<i32> = time_points
            .iter()
            .map(|item| {
                let mut it = item.split(':');
                return it.next().unwrap().parse::<i32>().unwrap() * 60
                    + it.next().unwrap().parse::<i32>().unwrap();
            })
            .collect();
        let max_diff = 24 * 60;
        timestamps.sort_unstable();
        timestamps.push(timestamps[0] + max_diff);
        (1..timestamps.len()).into_iter().fold(max_diff, |acc, i| {
            if timestamps[i] - timestamps[i - 1] < acc {
                timestamps[i] - timestamps[i - 1]
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        time_points: Vec<String>,
        ret: i32,
    }

    #[test]
    fn test_find_min_difference_simple() {
        let suites = vec![
            Suite {
                time_points: t1!["23:59", "00:00"],
                ret: 1,
            },
            Suite {
                time_points: t1!["00:00", "23:59", "00:00"],
                ret: 0,
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::find_min_difference(s.time_points));
        }
    }
}
