use super::*;


static EMPTY: i32 = 1000000001;
impl Solution {
  pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 1 {
      return nums;
    }
    let mut ret = vec![];
    let mut count1 = 0;
    let mut count2 = 0;
    let mut cand1 = EMPTY;
    let mut cand2 = EMPTY;
    // cand1和cand2不相互抵消，由于他们都大于1/3，所以当他们跟其他元素抵消时，总会有剩余
    // cand1和cand2（真）相互抵消的情况，只有在cand2（伪）有剩余的时候
    // 当把所有的cand2（伪）消掉时，cand2(真)可以取代cand2（伪）的位置
    // 而cand1实际抵消掉的是cand2（伪）的数量，相当于cand1没有和cand2抵消
    for &num in &nums {
      if cand1 == num {
        count1 += 1; 
      } else if cand2 == num {
        count2 += 1;
      } else if count1 == 0 {
        count1 = 1;
        cand1 = num;
      } else if count2 == 0 {
        count2 = 1;
        cand2 = num;
      } else {
        count1 -= 1;
        count2 -= 1;
      }
    }
    if count1 != 0 {
      count1 = 0;
      for &num in &nums {
        if num == cand1 {
          count1 += 1;
        }
      }
      if count1 * 3 > nums.len() as i32 {
        ret.push(cand1);
      }
    }
    if count2 != 0 {
      count2 = 0;
      for &num in &nums {
        if num == cand2 {
          count2 += 1;
        }
      }
      if count2 * 3 > nums.len() as i32 {
        ret.push(cand2);
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: Vec<i32>
  }

  #[test]
  fn test_majority_element_simple() {
    let suites = vec![
      Suite {
        nums: vec![3,2,3],
        ret: vec![3]
      },
      Suite {
        nums: vec![1],
        ret: vec![1]
      },
      Suite {
        nums: vec![1,2],
        ret: vec![1,2]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::majority_element(s.nums));
    }
  }
}