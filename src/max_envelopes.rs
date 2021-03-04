use super::Solution;

impl Solution {
  pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
    // 宽升序，高降序
    envelopes.sort_unstable_by(|a, b| {
      if a[0] == b[0] {
        b[1].cmp(&a[1])
      } else {
        a[0].cmp(&b[0])
      }
    });

    // tail[i]为第i+1个信封的高
    // 当信封的高大于tail[i]且小于tail[i + 1]时，更新tail[i + 1]的值，这样可以保证tail[i]尽可能小，让后面的信封更有可能装得进前面的信封
    // 假如此时的信封为j，那么则表示以信封j为最后一个的话，可以一共有i + 2个信封
    // 因为高是降序的，所以不存在信封j后的尺寸会影响确定信封j的在tail中的位置。
    let mut tail: Vec<i32> = vec![];
    for i in 0..envelopes.len() {
      let mut lo = 0;
      let mut li = tail.len() as i32 - 1;
      while lo <= li {
        let mid = lo + (li - lo) / 2;
        if envelopes[i][1] > tail[mid as usize] {
          lo = mid + 1;
        } else {
          li = mid - 1;
        }
      }
      if lo as usize >= tail.len() {
        tail.push(envelopes[i][1])
      } else {
        tail[lo as usize] = envelopes[i][1];
      }
    }
    tail.len() as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    envelopes: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn max_envelopes_simple() {
    let suites = vec![
      Suite {
        envelopes: Solution::t2_i(vec![&[5,4],&[6,4],&[6,7],&[2,3]]),
        ret: 3
      },
      Suite {
        envelopes: Solution::t2_i(vec![&[1,1],&[1,1],&[1,1]]),
        ret: 1
      }
    ];

    for s in suites {
      assert_eq!(Solution::max_envelopes(s.envelopes), s.ret);
    }
  }
}