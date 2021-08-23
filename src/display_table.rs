use super::*;

use std::collections::HashSet;
use std::collections::HashMap;

impl Solution {
  pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut dishes: HashSet<String> = HashSet::new();
    let mut table_to_dishes: HashMap<i32, HashMap<String, i32>> = HashMap::new();
    for order in orders {
      let dish = &order[2];
      dishes.insert(dish.to_owned());
      let table = order[1].parse::<i32>().unwrap();
      if !table_to_dishes.contains_key(&table) {
        table_to_dishes.insert(table, HashMap::new());
      }
      let counts = table_to_dishes.get_mut(&table).unwrap();
      if !counts.contains_key(dish) {
        counts.insert(dish.to_owned(), 0);
      }
      *counts.get_mut(dish).unwrap() += 1;
    }
    let mut dish_list: Vec<&String> = dishes.iter().collect();
    let mut table_list: Vec<&i32> = table_to_dishes.keys().collect();
    dish_list.sort();
    table_list.sort();
    let mut ret = vec![];
    ret.push(vec!["Table".to_owned()]);
    for &dish in &dish_list {
      ret[0].push(dish.to_owned());
    }
    for table in table_list {
      let mut row = vec![table.to_string()];
      let counts = table_to_dishes.get(table).unwrap();
      for &dish in &dish_list {
        if counts.contains_key(dish) {
          row.push(counts.get(dish).unwrap().to_string());
        } else {
          row.push("0".to_owned());
        }
      }
      ret.push(row);
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    orders: Vec<Vec<String>>,
    ret: Vec<Vec<String>>
  }

  #[test]
  fn test_display_table_simple() {
    let suites = vec![
      Suite {
        orders: t2![["David","3","Ceviche"],["Corina","10","Beef Burrito"],["David","3","Fried Chicken"],["Carla","5","Water"],["Carla","5","Ceviche"],["Rous","3","Ceviche"]],
        ret: t2![["Table","Beef Burrito","Ceviche","Fried Chicken","Water"],["3","0","2","1","0"],["5","0","1","0","1"],["10","1","0","0","0"]]
      },
      Suite {
        orders: t2![["James","12","Fried Chicken"],["Ratesh","12","Fried Chicken"],["Amadeus","12","Fried Chicken"],["Adam","1","Canadian Waffles"],["Brianna","1","Canadian Waffles"]],
        ret: t2![["Table","Canadian Waffles","Fried Chicken"],["1","2","0"],["12","0","3"]]
      },
      Suite {
        orders: t2![["Laura","2","Bean Burrito"],["Jhon","2","Beef Burrito"],["Melissa","2","Soda"]],
        ret: t2![["Table","Bean Burrito","Beef Burrito","Soda"],["2","1","1","1"]]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::display_table(s.orders));
    }
  }
}