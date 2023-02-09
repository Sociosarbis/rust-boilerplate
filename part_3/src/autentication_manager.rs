use std::collections::HashMap;

struct AuthenticationManager {
    time_to_live: i32,
    token_map: HashMap<String, i32>,
    token_list: Vec<(i32, String)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AuthenticationManager {
    fn new(timeToLive: i32) -> Self {
        return AuthenticationManager {
            time_to_live: timeToLive,
            token_map: HashMap::new(),
            token_list: vec![],
        };
    }

    fn generate(&mut self, token_id: String, mut current_time: i32) {
      current_time += self.time_to_live;
      self.token_map.insert(token_id.to_owned(), current_time);
      if self.token_list.is_empty() {
        self.token_list.push((current_time, token_id));
      } else {
        let mut l = 0;
        let mut r = self.token_list.len() - 1;
        while l <= r {
          let mid = (l + r) / 2;
          if current_time > self.token_list[mid].0 {
            l = mid + 1
          } else if current_time < self.token_list[mid].0 {
            if mid > 0 {
              r = mid - 1;
            } else {
              l = mid;
              break
            }
          } else {
            l = mid;
            break;
          }
        }
        if l >= self.token_list.len() {
          self.token_list.push((current_time, token_id));
        } else {
          self.token_list.insert(l, (current_time, token_id));
        }
      }
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
      if let Some(&t) = self.token_map.get(&token_id) {
        if t > current_time {
          let mut l = 0;
          let mut r = self.token_list.len() - 1;
          while l <= r {
            let mid = (l + r) / 2;
            if t > self.token_list[mid].0 {
              l = mid + 1;
            } else if t < self.token_list[mid].0 {
              r = mid - 1;
            } else {
              self.token_list.remove(mid);
              self.generate(token_id.to_owned(), current_time);
              break;
            }
          }
        }
      }
    }

    fn count_unexpired_tokens(&self, current_time: i32) -> i32 {
      if self.token_list.is_empty() {
        return 0;
      }
      let mut l = 0;
      let mut r = self.token_list.len() - 1;
      while l <= r {
        let mid = (l + r) / 2;
        if self.token_list[mid].0 <= current_time {
          l = mid + 1;
        } else {
          if mid > 0 && self.token_list[mid - 1].0 > current_time {
            r = mid - 1;
          } else {
            l = mid;
            break;
          }
        }
      }
      if l >= self.token_list.len() {
        return 0;
      }
      return (self.token_list.len() - l ) as i32
    }
}
