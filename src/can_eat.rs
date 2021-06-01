use super::Solution;

impl Solution {
  pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut need_eat = vec![0;candies_count.len() + 1];
    let mut acc: u64 = 0;
    let mut ret = vec![false;queries.len()];
    for i in 0..candies_count.len() {
      acc += candies_count[i] as u64;
      need_eat[i + 1] = acc;
    }
    for i in 0..queries.len() {
      let q = &queries[i];
      let min = q[1];
      let max = q[2] as u64 * (q[1] as u64 + 1);
      let at_least_eat = need_eat[q[0] as usize];
      let at_most_eat = need_eat[q[0] as usize + 1];
      if !(at_least_eat >= max  || min as u64 >= at_most_eat) {
        ret[i] = true;
      }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    candies_count: Vec<i32>,
    queries: Vec<&'a[i32]>,
    ret: Vec<bool>
  }

  #[test]
  fn test_can_eat() {
    let suites = vec![
      Suite {
        candies_count: vec![7,4,5,3,8], 
        queries: vec![&[0,2,2],&[4,2,4],&[2,13,1000000000]],
        ret: vec![true,false,true]
      },
      Suite {
        candies_count: vec![5,2,6,4,1], 
        queries: vec![&[3,1,2],&[4,10,3],&[3,10,100],&[4,100,30],&[1,3,1]],
        ret: vec![false,true,true,false,false]
      },
      Suite {
        candies_count: vec![46,5,47,48,43,34,15,26,11,25,41,47,15,25,16,50,32,42,32,21,36,34,50,45,46,15,46,38,50,12,3,26,26,16,23,1,4,48,47,32,47,16,33,23,38,2,19,50,6,19,29,3,27,12,6,22,33,28,7,10,12,8,13,24,21,38,43,26,35,18,34,3,14,48,50,34,38,4,50,26,5,35,11,2,35,9,11,31,36,20,21,37,18,34,34,10,21,8,5],
        queries: vec![&[85, 54, 42]],
        ret: vec![true]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::can_eat(s.candies_count, Solution::t2_i(s.queries)));
    }
  }
}