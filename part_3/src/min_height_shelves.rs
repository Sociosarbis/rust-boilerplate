use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let mut dp: HashMap<(i32, i32), i32> = HashMap::new();
    dp.insert((shelf_width, 0), 0);
    for book in books {
      let mut next_dp: HashMap<(i32, i32), i32> = HashMap::new();
      for ((w, max_h), h) in dp {
        let mut items = vec![((book[0], book[1]), book[1] + h)];
        if w + book[0] <= shelf_width {
          let next_max = max_h.max(book[1]); 
          items.push(((w + book[0], max_h.max(book[1])), h - max_h + next_max));
        };
        for item in items {
          if let Some(h) = next_dp.get_mut(&item.0) {
            if *h > item.1 {
              *h = item.1
            }
          } else {
            next_dp.insert(item.0, item.1);
          }
        }
      }
      dp = next_dp;
    }
    dp.into_iter().map(|item| item.1).min().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    books: Vec<Vec<i32>>,
    shelf_width: i32,
    ret: i32
  }

  #[test]
  fn test_min_height_shelves_simple() {
    let suites = vec![
      Suite {
        books: t2_i![[1,1],[2,3],[2,3],[1,1],[1,1],[1,1],[1,2]],
        shelf_width: 4,
        ret: 6
      },
      Suite {
        books: t2_i![[1,3],[2,4],[3,2]],
        shelf_width: 6,
        ret: 4
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_height_shelves(s.books, s.shelf_width));
    }
  }
}