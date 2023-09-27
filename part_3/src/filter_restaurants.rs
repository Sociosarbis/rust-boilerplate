use super::*;

impl Solution {
  pub fn filter_restaurants(
    restaurants: Vec<Vec<i32>>,
    vegan_friendly: i32,
    max_price: i32,
    max_distance: i32,
  ) -> Vec<i32> {
    let mut ret: Vec<(i32, i32)> = restaurants
      .into_iter()
      .filter(|r| {
        r[3] <= max_price
          && r[4] <= max_distance
          && if vegan_friendly == 1 { r[2] == 1 } else { true }
      })
      .map(|r| (r[1], r[0]))
      .collect();
    ret.sort_unstable();
    ret.reverse();
    ret.into_iter().map(|r| r.1).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    restaurants: Vec<Vec<i32>>,
    vegan_friendly: i32,
    max_price: i32,
    max_distance: i32,
    ret: Vec<i32>,
  }

  #[test]
  fn test_filter_restaurants_simple() {
    let suites = vec![
      Suite {
        restaurants: t2_i![
          [1, 4, 1, 40, 10],
          [2, 8, 0, 50, 5],
          [3, 8, 1, 30, 4],
          [4, 10, 0, 10, 3],
          [5, 1, 1, 15, 1]
        ],
        vegan_friendly: 1,
        max_price: 50,
        max_distance: 10,
        ret: vec![3, 1, 5],
      },
      Suite {
        restaurants: t2_i![
          [1, 4, 1, 40, 10],
          [2, 8, 0, 50, 5],
          [3, 8, 1, 30, 4],
          [4, 10, 0, 10, 3],
          [5, 1, 1, 15, 1]
        ],
        vegan_friendly: 0,
        max_price: 50,
        max_distance: 10,
        ret: vec![4, 3, 2, 1, 5],
      },
      Suite {
        restaurants: t2_i![
          [1, 4, 1, 40, 10],
          [2, 8, 0, 50, 5],
          [3, 8, 1, 30, 4],
          [4, 10, 0, 10, 3],
          [5, 1, 1, 15, 1]
        ],
        vegan_friendly: 0,
        max_price: 30,
        max_distance: 3,
        ret: vec![4, 5],
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::filter_restaurants(s.restaurants, s.vegan_friendly, s.max_price, s.max_distance)
      );
    }
  }
}
