use super::*;

impl Solution {
    pub fn max_total_fruits(mut fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let mut l = 0;
        let mut r = fruits.len() - 1;
        while l <= r {
            let mid = (l + r) >> 1;
            if fruits[mid][0] < start_pos {
                l = mid + 1;
            } else {
                if mid > 0 && fruits[mid - 1][0] >= start_pos {
                    r = mid - 1;
                } else {
                    l = mid;
                    break;
                }
            }
        }
        let mut left = vec![(0, 0); l + 1];
        for i in (0..l).rev() {
            left[l - i].0 = start_pos - fruits[i][0];
            left[l - i].1 = left[l - i - 1].1 + fruits[i][1];
        }
        let mut temp = 0;
        let mut ret = 0;
        if l >= fruits.len() {
            fruits.push(vec![start_pos, 0]);
        } else if fruits[l][0] != start_pos {
          fruits.insert(l, vec![start_pos, 0]);
        }
        let mut left_index = left.len() - 1;
        for i in l..fruits.len() {
            if fruits[i][0] - start_pos <= k {
                temp += fruits[i][1];
                let mut next_ret = temp;
                let right_cost = fruits[i][0] - start_pos;
                let rest_step = (if k > right_cost * 2 {
                    k - right_cost * 2
                } else {
                    0
                })
                .max((k - right_cost) / 2);
                let mut l = 1;
                let mut r = left_index;
                while l <= r {
                  let mid = (l + r) >> 1;
                  if left[mid].0 > rest_step {
                    r = mid - 1;
                  } else {
                    if mid + 1 < left.len() && left[mid + 1].0 <= rest_step {
                      l = mid + 1;
                    } else {
                      next_ret += left[mid].1;
                      left_index = mid;
                      break;
                    }
                  }
                }
                if next_ret > ret {
                    ret = next_ret;
                }
            } else {
                break;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        fruits: Vec<Vec<i32>>,
        start_pos: i32,
        k: i32,
        ret: i32,
    }

    #[test]
    fn test_max_total_fruits_simple() {
        let suites = vec![
            Suite {
                fruits: t2_i![[2, 8], [6, 3], [8, 6]],
                start_pos: 5,
                k: 4,
                ret: 9,
            },
            Suite {
                fruits: t2_i![[0, 9], [4, 1], [5, 7], [6, 2], [7, 4], [10, 9]],
                start_pos: 5,
                k: 4,
                ret: 14,
            },
            Suite {
                fruits: t2_i![[0, 3], [6, 4], [8, 5]],
                start_pos: 3,
                k: 2,
                ret: 0,
            },
            Suite {
              fruits: t2_i![[1,2],[2,3],[4,1],[6,6],[8,1],[21,1],[24,2],[26,1],[29,10]],
              start_pos: 27,
              k: 23,
              ret: 15
            }
        ];

        for s in suites {
            assert_eq!(
                s.ret,
                Solution::max_total_fruits(s.fruits, s.start_pos, s.k)
            );
        }
    }
}
