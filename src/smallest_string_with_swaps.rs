use super::Solution;

impl Solution {
  pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
    if s.is_empty() || pairs.is_empty() { return s; }
    let mut groups: Vec<usize> = vec![0;s.len()];
    let mut g2m: Vec<Vec<usize>> = vec![vec![]];
    let chars: Vec<char> = s.chars().collect();
    let mut ret: Vec<char> = vec![];
    for p in pairs {
      let l = p[0] as usize;
      let r = p[1] as usize;
      if l != r {
        let left = groups[l];
        let right = groups[r];
        if left != 0 {
          if right != 0 {
            if left != right {
              let s = if g2m[right].len() > g2m[left].len() { left } else { right };
              let t = if s == left { right } else { left };
              for j in 0..g2m[s].len() {
                let index = {
                  g2m[s][j]
                };
                groups[index] = t;
                g2m[t].push(index);
              }
              g2m[s].clear();
            }
          } else {
            groups[r] = groups[l];
            g2m[left].push(r);
          }
        } else if right != 0 {
          groups[l] = right;
          g2m[right].push(l);
        } else {
          groups[l] = g2m.len();
          groups[r] = g2m.len();
          g2m.push(vec![l, r]);
        }
      }
    }

    for it in &mut g2m {
      if !it.is_empty() {
        it.sort_unstable_by(|&a, &b| { (chars[b] as i32).cmp(&(chars[a] as i32)) });
      }
    }

    for i in 0..groups.len() {
      if groups[i] == 0 {
        ret.push(chars[i]);
      } else {
        ret.push(chars[g2m[groups[i]].pop().unwrap()]);
      }
    }
    
    ret.into_iter().collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    s: String,
    pairs: Vec<Vec<i32>>,
    ret: String
  }

  #[test]
  fn smallest_string_with_swaps_simple() {
    let suites: Vec<Suite> = vec![
      Suite {
        s: "dcab".to_owned(),
        pairs: vec![vec![0,3],vec![1,2]],
        ret: "bacd".to_owned()
      },
      Suite {
        s: "dcab".to_owned(),
        pairs: vec![vec![0,3],vec![1,2],vec![0,2]],
        ret: "abcd".to_owned()
      },
      Suite {
        s: "cba".to_owned(),
        pairs: vec![vec![0,1],vec![1,2]],
        ret: "abc".to_owned()
      },
      Suite {
        s: "qdwyt".to_owned(),
        pairs: vec![vec![2,3],vec![3,2],vec![0,1],vec![4,0],vec![3,2]],
        ret: "dqwyt".to_owned(),
      },
      Suite {
        s: "udyyek".to_owned(),
        pairs: vec![vec![3,3],vec![3,0],vec![5,1],vec![3,1],vec![3,4],vec![3,5]],
        ret: "deykuy".to_owned()
      },
      Suite {
        s: "tklkxyizmlqf".to_owned(),
        pairs: vec![vec![2,10],vec![3,5],vec![8,11],vec![1,2],vec![10,6],vec![4,1],vec![1,10],vec![5,8],vec![8,3],vec![10,4],vec![7,3],vec![10,11]],
        ret: "tfikklmqxlyz".to_owned()
      },
      Suite {
        s: "cegfxvulsxakw".to_owned(),
        pairs: vec![vec![6,6],vec![5,7],vec![11,4],vec![0,0],vec![6,2],vec![6,7],vec![0,7],vec![4,0],vec![3,1],vec![2,9],vec![4,7],vec![8,6],vec![9,0]],
        ret: "cegfklsuvxaxw".to_owned()
      }
    ];

    for s in suites {
      assert_eq!(Solution::smallest_string_with_swaps(s.s, s.pairs), s.ret)
    }
  }
}