use super::*;

use std::collections::HashMap;

impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut name_to_times: HashMap<String, Vec<i32>> = HashMap::new();
        for i in 0..key_time.len() {
            let hm = key_time[i]
                .split(":")
                .enumerate()
                .map(|(i, v)| (60 as i32).pow(1 - i as u32) * v.parse::<i32>().unwrap())
                .fold(0, |acc, item| acc + item);
            if let Some(v) = name_to_times.get_mut(&key_name[i]) {
                v.push(hm);
            } else {
                name_to_times.insert(key_name[i].to_owned(), vec![hm]);
            }
        }
        let mut ret = vec![];
        for (k, v) in &mut name_to_times {
          if v.len() >= 3 {
            v.sort();
            for i in 0..v.len() - 2 {
              if v[i] + 60 >= v[i + 2] {
                ret.push(k.to_owned());
                break
              }
            }
          }
        }
        ret.sort();
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        key_name: Vec<String>,
        key_time: Vec<String>,
        ret: Vec<String>,
    }

    #[test]
    fn test_alert_names_simple() {
        let suites = vec![
            Suite {
                key_name: t1!["daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"],
                key_time: t1!["10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00"],
                ret: t1!["daniel"],
            },
            Suite {
                key_name: t1!["alice", "alice", "alice", "bob", "bob", "bob", "bob"],
                key_time: t1!["12:01", "12:00", "18:00", "21:00", "21:20", "21:30", "23:00"],
                ret: t1!["bob"],
            },
            Suite {
                key_name: t1!["a", "a", "a", "a", "a", "a", "b", "b", "b", "b", "b"],
                key_time: t1![
                    "23:27", "03:14", "12:57", "13:35", "13:18", "21:58", "22:39", "10:49",
                    "19:37", "14:14", "10:41"
                ],
                ret: t1!["a"],
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::alert_names(s.key_name, s.key_time))
        }
    }
}
