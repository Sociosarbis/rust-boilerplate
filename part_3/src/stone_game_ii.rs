use super::*;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let mut prefix = vec![0; piles.len() + 1];
        for (i, &v) in piles.iter().enumerate() {
            prefix[i + 1] = prefix[i] + v;
        }
        let mut dp = vec![vec![vec![0; piles.len()]; piles.len()]; 2];
        Solution::dfs(&prefix, &mut dp, 0, 0, 0, 1)
    }

    fn dfs(
        prefix: &Vec<i32>,
        dp: &mut Vec<Vec<Vec<i32>>>,
        index: usize,
        temp: i32,
        i: usize,
        m: usize,
    ) -> i32 {
        if i + 2 * m >= prefix.len() - 1 {
            if index == 0 {
                return temp + prefix[prefix.len() - 1] - prefix[i];
            } else {
                return temp;
            }
        }
        if dp[index][i][m] != 0 {
          return dp[index][i][m] + temp;
        }
        if index == 0 {
            let mut max = 0;
            for j in i..i + 2 * m {
                let res = Solution::dfs(
                    prefix,
                    dp,
                    1 - index,
                    temp + prefix[j + 1] - prefix[i],
                    j + 1,
                    if j - i + 1 <= m { m } else { j - i + 1 },
                );
                if res > max {
                    max = res;
                }
            }
            dp[index][i][m] = max - temp;
            max
        } else {
            let mut min = prefix[prefix.len() - 1];
            for j in i..i + 2 * m {
                let res = Solution::dfs(
                    prefix,
                    dp,
                    1 - index,
                    temp,
                    j + 1,
                    if j - i + 1 <= m { m } else { j - i + 1 },
                );
                if res < min {
                    min = res;
                }
            }
            dp[index][i][m] = min - temp;
            min
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        piles: Vec<i32>,
        ret: i32,
    }

    #[test]
    fn test_stone_game_ii_simple() {
        let suites = vec![
            Suite {
                piles: vec![2, 7, 9, 4, 4],
                ret: 10,
            },
            Suite {
                piles: vec![1, 2, 3, 4, 5, 100],
                ret: 104,
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::stone_game_ii(s.piles));
        }
    }
}
