use super::*;


impl Solution {
  pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ranges = vec![];
    let mut l = None;
    let mut i = 0;
    for c in s.chars() {
      match c {
        '|' => {
          if let Some(v) = l.as_mut() {
            ranges.push((*v, i));
            *v = i;
          } else {
            l = Some(i);
          }
        },
        _ => {}
      }
      i += 1;
    }
    let mut ret = vec![0;queries.len()];
    if !ranges.is_empty() {
      let mut prefix_sum = vec![0;ranges.len()];
      let mut temp = 0;
      i = 0;
      for r in &ranges {
        temp += r.1 - r.0 - 1;
        prefix_sum[i] = temp as i32;
        i += 1;
      }
      i = 0;
      for q in &queries {
        let mut l = 0;
        let mut r = ranges.len() - 1;
        while l <= r {
          let mid = (l + r) >> 1;
          if ranges[mid].0 > q[0] as usize {
            if mid > 0 && ranges[mid - 1].0 >= q[0] as usize {
              r = mid - 1;
            } else {
              l = mid;
              break;
            }
          } else if ranges[mid].0 < q[0] as usize {
            l = mid + 1;
          } else {
            l = mid;
            break;
          }
        }
        let left_bound = l;
        if left_bound <= r {
          r = ranges.len() - 1;
          while l <= r {
            let mid = (l + r) >> 1;
            if ranges[mid].1 > q[1] as usize {
              if mid > 0 {
                r = mid - 1;
              } else {
                l = ranges.len();
                break;
              }
            } else if ranges[mid].1 < q[1] as usize {
              if mid + 1 < ranges.len() && ranges[mid + 1].1 <= q[1] as usize {
                l = mid + 1;
              } else {
                l = mid;
                break;
              }
            } else {
              l = mid;
              break;
            }
          }
          if l <= r {
            ret[i] = prefix_sum[l] - if left_bound > 0 { prefix_sum[left_bound - 1] } else { 0 };
          }
        }
        i += 1;
      }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    s: &'a str,
    queries: Vec<Vec<i32>>,
    ret: Vec<i32>
  }

  #[test]
  fn test_plates_between_candles_simple() {
    let suites = vec![
      Suite {
        s: "**|**|***|",
        queries: t2_i![[2,5],[5,9]],
        ret: vec![2,3]
      },
      Suite {
        s: "***|**|*****|**||**|*",
        queries: t2_i![[1,17],[4,5],[14,17],[5,11],[15,16]],
        ret: vec![9,0,0,0,0]
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::plates_between_candles(s.s.to_string(), s.queries));
    }
  }
}