use super::*;

impl Solution {
    pub fn num_moves_stones_ii(mut stones: Vec<i32>) -> Vec<i32> {
        stones.sort();
        let n = stones.len();
        let mut low = n as i32;
        let mut i = 0;
        for j in 0..n {
            while stones[j] - stones[i] + 1 > n as i32 {
                i += 1;
            }
            let already_store = (j - i + 1) as i32;
            if already_store == n as i32 - 1 && stones[j] - stones[i] + 1 == n as i32 - 1 {
                low = low.min(2);
            } else {
                low = low.min(n as i32 - already_store);
            }
        }
        // 总有方法去把[1:n-1]和[0:n-2]下的空格都填一遍
        return vec![
            low,
            (stones[n - 1] - stones[1] - n as i32 + 2)
                .max(stones[n - 2] - stones[0] - n as i32 + 2),
        ];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        stones: Vec<i32>,
        ret: Vec<i32>,
    }

    #[test]
    fn test_num_moves_stones_ii_simple() {
        let suites = vec![
            Suite {
                stones: vec![7, 4, 9],
                ret: vec![1, 2],
            },
            Suite {
                stones: vec![6, 5, 4, 3, 10],
                ret: vec![2, 3],
            },
            Suite {
                stones: vec![8, 7, 6, 5, 1000000000],
                ret: vec![2, 999999991],
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::num_moves_stones_ii(s.stones));
        }
    }
}
