use super::*;

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut nums = vec![a, b, c];
        nums.sort();
        let left_gap = nums[1] - nums[0] - 1;
        let right_gap = nums[2] - nums[1] - 1;
        let min_gap = (left_gap).min(right_gap);
        vec![
            if min_gap > 1 {
                2
            } else if left_gap != 0 || right_gap != 0 {
                1
            } else {
                0
            },
            nums[2] - nums[0] - 2,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        a: i32,
        b: i32,
        c: i32,
        ret: Vec<i32>,
    }

    #[test]
    fn test_num_moves_stones_simple() {
        let suites = vec![
            Suite {
                a: 1,
                b: 2,
                c: 5,
                ret: vec![1, 2],
            },
            Suite {
                a: 4,
                b: 3,
                c: 2,
                ret: vec![0, 0],
            },
            Suite {
                a: 3,
                b: 5,
                c: 1,
                ret: vec![1, 2],
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::num_moves_stones(s.a, s.b, s.c));
        }
    }
}
