use super::*;

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut prefix = vec![0; hours.len() + 1];
        for i in 0..hours.len() {
            prefix[i + 1] = prefix[i] + if hours[i] > 8 { 1 } else { -1 };
        }
        let mut j = hours.len();
        let mut max = prefix[0];
        while j != 0 && prefix[j] <= max {
            j -= 1;
        }
        let mut ret = j - 0;
        for i in 1..hours.len() {
            if i + ret >= prefix.len() {
                break;
            }
            if prefix[i] < max {
                for k in (i + ret + 1..prefix.len()).rev() {
                    if prefix[k] > prefix[i] {
                        ret = k - i;
                        break;
                    }
                }
                max = hours[i];
            }
        }
        ret as i32
    }

    pub fn longest_wpi_best(hours: Vec<i32>) -> i32 {
        let mut prefix = vec![0; hours.len() + 1];
        for i in 0..hours.len() {
            prefix[i + 1] = prefix[i] + if hours[i] > 8 { 1 } else { -1 };
        }
        let mut stack = vec![0];
        for i in 1..prefix.len() {
            if let Some(&j) = stack.last() {
                if prefix[i] < prefix[j] {
                    stack.push(i);
                }
            }
        }
        let mut ret = 0;
        for i in (0..prefix.len()).rev() {
            for j in (0..stack.len()).rev() {
                if stack[j] >= i {
                    stack.pop();
                }
            }
            while !stack.is_empty() && prefix[i] > prefix[*stack.last().unwrap()] {
                if let Some(j) = stack.pop() {
                    if i - j > ret {
                        ret = i - j;
                    }
                }
            }
        }
        ret as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        hours: Vec<i32>,
        ret: i32,
    }

    #[test]
    fn test_longest_wpi_simple() {
        let suites = vec![
            Suite {
                hours: vec![9, 9, 6, 0, 6, 6, 9],
                ret: 3,
            },
            Suite {
                hours: vec![6, 6, 6],
                ret: 0,
            },
            Suite {
                hours: vec![
                    11, 2, 4, 14, 2, 15, 7, 10, 1, 16, 9, 0, 2, 8, 4, 14, 6, 12, 2, 8, 6, 4, 14,
                    13, 7, 16, 14, 2, 3, 2, 8, 3, 12, 3, 3, 9, 14, 1, 5, 3, 12, 0, 15, 5, 0, 2, 3,
                    16, 7, 2, 1, 1, 4, 9, 0, 11, 9, 16, 15, 7, 0, 5, 6, 4, 12, 1, 1, 2, 13, 8, 3,
                    9, 12, 9, 3, 11, 4, 14, 7, 5, 16, 0, 11, 8, 8, 14, 1, 5, 0, 6, 5, 8, 10, 15, 9,
                    14, 16, 11, 1, 13,
                ],
                ret: 29,
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::longest_wpi_best(s.hours));
        }
    }
}
