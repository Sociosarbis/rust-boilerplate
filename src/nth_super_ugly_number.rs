use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
    let mut nums = vec![0;n as usize];
    nums[0] = 1;
    let mut queue = primes.clone();
    let mut num_to_prime: HashMap<i32, usize> = HashMap::new();
    let mut prime_to_index: Vec<usize> = vec![0;primes.len()];
    for i in 0..primes.len() {
      num_to_prime.insert(primes[i], i);
    }
    let mut i = 1;
    while i < n as usize {
      nums[i] = queue.remove(0);
      let prime_index = *num_to_prime.get(&nums[i]).unwrap();
      let max_val = 2147483647 / primes[prime_index];
      
      let next_num = {
        if nums[prime_to_index[prime_index] + 1] > max_val {
          return -1;
        }
        let mut temp = primes[prime_index] * nums[prime_to_index[prime_index] + 1];
        while num_to_prime.contains_key(&temp) {
          prime_to_index[prime_index] += 1;
          if nums[prime_to_index[prime_index] + 1] > max_val {
            return -1;
          }
          temp = primes[prime_index] * nums[prime_to_index[prime_index] + 1];
        }
        temp
      };
      if next_num != -1 {
        num_to_prime.remove(&nums[i]);
        num_to_prime.insert(next_num, prime_index);
        prime_to_index[prime_index] += 1;
        let index = Solution::binary_search(&queue, next_num, true) as usize;
        if index < queue.len() {
          queue.insert(index, next_num);
        } else {
          queue.push(next_num);
        }
      }
      i += 1;
    }
    *nums.last().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    primes: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_nth_super_ugly_number_simple() {
    let suites = vec![
      Suite {
        n: 12,
        primes: vec![2,7,13,19],
        ret: 32,
      },
      Suite {
        n: 1,
        primes: vec![2,3,5],
        ret: 1,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::nth_super_ugly_number(s.n, s.primes));
    }
  }
}