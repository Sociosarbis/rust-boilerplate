use super::*;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut map: Vec<Vec<i32>> = vec![vec![];n as usize];
        for (i, &leader) in manager.iter().enumerate() {
          if leader != -1 {
            map[leader as usize].push(i as i32); 
          }
        }

        Solution::num_of_minutes_dfs(&map, &inform_time, head_id)
    }

    fn num_of_minutes_dfs(
        map: &Vec<Vec<i32>>,
        inform_time: &Vec<i32>,
        leader: i32,
    ) -> i32 {
        let subs = &map[leader as usize];
        if subs.is_empty() {
          0
        } else {
          let temp = inform_time[leader as usize];
          temp + subs
              .iter()
              .map(|sub| Solution::num_of_minutes_dfs(map, inform_time, *sub))
              .max()
              .unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        n: i32,
        head_id: i32,
        manager: Vec<i32>,
        inform_time: Vec<i32>,
        ret: i32,
    }

    #[test]
    fn test_num_of_minutes_simple() {
        let suites = vec![
            Suite {
                n: 1,
                head_id: 0,
                manager: vec![-1],
                inform_time: vec![0],
                ret: 0,
            },
            Suite {
                n: 6,
                head_id: 2,
                manager: vec![2, 2, -1, 2, 2, 2],
                inform_time: vec![0, 0, 1, 0, 0, 0],
                ret: 1,
            },
        ];

        for s in suites {
            assert_eq!(
                s.ret,
                Solution::num_of_minutes(s.n, s.head_id, s.manager, s.inform_time)
            );
        }
    }
}
