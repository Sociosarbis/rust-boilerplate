use super::*;

impl Solution {
  pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let mut i = 0;
    let mut ret = vec![];
    let mut width = 0;
    for j in 0..words.len() {
      let next_width = width + words[j].len() as i32;
      if next_width + (j - i) as i32 <= max_width {
        width = next_width;
      } else {
        let mut s = words[i].to_owned();
        let delta = max_width - width;
        if i + 1 == j {
          s.push_str(str::repeat(" ", delta as usize).as_str());
        } else {
          let gap = delta / (j - i - 1) as i32;
          let num = delta % (j - i - 1) as i32;
          for k in i + 1..j {
            s.push_str(str::repeat(" ", (if k <= i + num as usize { gap + 1 } else { gap }) as usize).as_str());
            s.push_str(words[k].as_str());
          }
        }
        ret.push(s);
        width = words[j].len() as i32;
        i = j;
      }
      if j + 1 == words.len() {
        let mut s = String::new();
        let delta = max_width - width;
        for k in i..words.len() {
          s.push_str(words[k].as_str());
          s.push_str(str::repeat(" ", (if k + 1 == words.len() { delta - (words.len() - i) as i32 + 1 } else { 1 }) as usize).as_str());
        }
        ret.push(s);
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    words: Vec<String>,
    max_width: i32,
    ret: Vec<String>
  }

  #[test]
  fn test_full_justify_simple() {
    let suites = vec![
      Suite {
        words: t1!["This", "is", "an", "example", "of", "text", "justification."],
        max_width: 16,
        ret: t1![
          "This    is    an",
          "example  of text",
          "justification.  "
       ]
      },
      Suite {
        words: t1!["What","must","be","acknowledgment","shall","be"],
        max_width: 16,
        ret: t1![
          "What   must   be",
          "acknowledgment  ",
          "shall be        "
        ]
      },
      Suite {
        words: t1!["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"],
        max_width: 20,
        ret: t1![
          "Science  is  what we",
          "understand      well",
          "enough to explain to",
          "a  computer.  Art is",
          "everything  else  we",
          "do                  "
        ]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::full_justify(s.words, s.max_width));
    }
  }
}