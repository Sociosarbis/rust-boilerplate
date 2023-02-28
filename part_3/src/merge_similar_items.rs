use super::*;

impl Solution {
  pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut counter = [0;1001];
    for item in items1 {
      counter[item[0] as usize] = item[1];
    }
    for item in items2 {
      counter[item[0] as usize] += item[1];
    }
    let mut ret = vec![];
    for (i, &v) in counter.iter().enumerate() {
      if v > 0 {
        ret.push(vec![i as i32, v]);
      }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    items1: Vec<Vec<i32>>,
    items2: Vec<Vec<i32>>,
    ret: Vec<Vec<i32>>
  }

  #[test]
  fn test_merge_similar_items_simple() {
    let suites = vec![
      Suite {
        items1: t2_i![[1,1],[4,5],[3,8]],
        items2: t2_i![[3,1],[1,5]],
        ret: t2_i![[1,6],[3,9],[4,5]]
      },
      Suite {
        items1: t2_i![[1,1],[3,2],[2,3]],
        items2: t2_i![[2,1],[3,2],[1,3]],
        ret: t2_i![[1,4],[2,4],[3,4]]
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::merge_similar_items(s.items1, s.items2));
    }
  }
}