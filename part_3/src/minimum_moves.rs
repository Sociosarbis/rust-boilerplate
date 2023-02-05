use super::*;

use std::collections::HashSet;

impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut m: HashSet<(usize, usize, i8)> = HashSet::new();
        m.insert((0, 1, 0));
        let mut bfs: Vec<(usize, usize, i8)> = vec![(0, 1, 0)];
        let size = grid.len();
        let mut r = 0;
        let insert =
            |m: &mut HashSet<(usize, usize, i8)>, bfs: &mut Vec<(usize, usize, i8)>, item| {
                if !m.contains(&item) {
                    m.insert(item);
                    bfs.push(item);
                }
            };
        while !bfs.is_empty() {
            let n = bfs.len();
            for i in 0..n {
                let (i, j, s) = bfs[i];
                if i == size - 1 && j == size - 1 && s == 0 {
                    return r;
                }
                match s {
                    0 => {
                        if j + 1 < size && grid[i][j + 1] == 0 {
                            insert(&mut m, &mut bfs, (i, j + 1, 0));
                        }
                        if i + 1 < size && grid[i + 1][j - 1] == 0 && grid[i + 1][j] == 0 {
                            insert(&mut m, &mut bfs, (i + 1, j, 0));
                            insert(&mut m, &mut bfs, (i + 1, j - 1, 1));
                        }
                    }
                    1 => {
                        if i + 1 < size && grid[i + 1][j] == 0 {
                          insert(&mut m, &mut bfs, (i + 1, j, 1));
                        }
                        if j + 1 < size && grid[i][j + 1] == 0 && grid[i - 1][j + 1] == 0 {
                          insert(&mut m, &mut bfs, (i, j + 1, 1));
                          insert(&mut m, &mut bfs, (i - 1, j + 1, 0));
                        }
                    }
                    _ => unreachable!(),
                }
            }
            bfs.drain(0..n);
            r += 1;
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        grid: Vec<Vec<i32>>,
        ret: i32,
    }

    #[test]
    fn test_minimum_moves_simple() {
        let suites = vec![
            Suite {
                grid: t2_i![
                    [0, 0, 0, 0, 0, 1],
                    [1, 1, 0, 0, 1, 0],
                    [0, 0, 0, 0, 1, 1],
                    [0, 0, 1, 0, 1, 0],
                    [0, 1, 1, 0, 0, 0],
                    [0, 1, 1, 0, 0, 0]
                ],
                ret: 11,
            },
            Suite {
                grid: t2_i![
                    [0, 0, 1, 1, 1, 1],
                    [0, 0, 0, 0, 1, 1],
                    [1, 1, 0, 0, 0, 1],
                    [1, 1, 1, 0, 0, 1],
                    [1, 1, 1, 0, 0, 1],
                    [1, 1, 1, 0, 0, 0]
                ],
                ret: 9,
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::minimum_moves(s.grid));
        }
    }
}
