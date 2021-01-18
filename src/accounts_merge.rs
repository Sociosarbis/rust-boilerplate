use super::Solution;
use std::collections::HashMap;

impl Solution {
  pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut owners: Vec<Vec<String>> = vec![vec![]];
    for a in accounts {
      let mails = a[1..a.len()].to_owned();
      let mut group = 0;
      for i in 1..a.len() {
        if let Some(&n) = map.get(&a[i]) {
          group = n;
          break;
        }
      }
      if group == 0 {
        let mut new_account = vec![a[0].to_owned()];
        let id = owners.len();
        for m in mails {
          if map.insert(m.to_owned(), id) == None {
            new_account.push(m);
          };
        }
        owners.push(new_account);
      } else {
        for j in 0..mails.len() {
          let m = mails[j].to_owned();
          let has_old_group = {
            map.contains_key(&m)
          };
          if !has_old_group {
            owners[group].push(m.to_owned());
            map.insert(m, group);
          } else {
            let old_group = {
              *map.get(&m).unwrap()
            };
            if old_group != group {
              for i in 1..owners[old_group].len() {
                let m = owners[old_group][i].to_owned();
                owners[group].push(m.to_owned());
                map.insert(m, group);
              }
              owners[old_group].clear();
            }
          }
        }
      }
    }
    owners.into_iter().filter(|item| { !item.is_empty() }).map(|item| {
      let mut new_item = vec![item[0].to_owned()];
      let mut mails = item[1..item.len()].to_owned();
      mails.sort();
      new_item.append(&mut mails);
      new_item
    }).collect()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    accounts: Vec<Vec<String>>,
    ret: Vec<Vec<String>>,
  }

  #[test]
  fn accounts_merge_simple() {
    let suites = vec![
      Suite {
        accounts: Solution::t2(vec![vec!["John", "johnsmith@mail.com", "john00@mail.com"], vec!["John", "johnnybravo@mail.com"], vec!["John", "johnsmith@mail.com", "john_newyork@mail.com"], vec!["Mary", "mary@mail.com"]]),
        ret: Solution::t2(vec![vec!["John", "john00@mail.com", "john_newyork@mail.com", "johnsmith@mail.com"],  vec!["John", "johnnybravo@mail.com"], vec!["Mary", "mary@mail.com"]])
      }];

    for s in suites {
      assert_eq!(Solution::accounts_merge(s.accounts), s.ret)
    }
  }
}