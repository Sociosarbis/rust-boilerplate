use super::*;

impl Solution {
  pub fn entity_parser(text: String) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut ret = String::new();
    let mut i = 0;
    while i < chars.len() {
      if chars[i] == '&' {
        if i + 3 < chars.len() {
          if chars[i + 3] == ';' {
            if chars[i + 2] == 't' {
              if chars[i + 1] == 'g' {
                ret.push('>');
                i += 4;
                continue;
              } else if chars[i + 1] == 'l' {
                ret.push('<');
                i += 4;
                continue;
              }
            }
          }
          if i + 4 < chars.len() {
            if chars[i + 1] == 'a'
              && chars[i + 2] == 'm'
              && chars[i + 3] == 'p'
              && chars[i + 4] == ';'
            {
              ret.push('&');
              i += 5;
              continue;
            }

            if i + 5 < chars.len() {
              if chars[i + 5] == ';' {
                if chars[i + 1] == 'q'
                  && chars[i + 2] == 'u'
                  && chars[i + 3] == 'o'
                  && chars[i + 4] == 't'
                {
                  ret.push('"');
                  i += 6;
                  continue;
                }
                if chars[i + 1] == 'a'
                  && chars[i + 2] == 'p'
                  && chars[i + 3] == 'o'
                  && chars[i + 4] == 's'
                {
                  ret.push('\'');
                  i += 6;
                  continue;
                }
              } else if i + 6 < chars.len() {
                if chars[i + 1] == 'f'
                  && chars[i + 2] == 'r'
                  && chars[i + 3] == 'a'
                  && chars[i + 4] == 's'
                  && chars[i + 5] == 'l'
                  && chars[i + 6] == ';'
                {
                  ret.push('/');
                  i += 7;
                  continue;
                }
              }
            }
          }
        }
      }
      ret.push(chars[i]);
      i += 1;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    text: String,
    ret: String,
  }

  #[test]
  fn test_entity_parser_simple() {
    let suites = vec![
      Suite {
        text: "&amp; is an HTML entity but &ambassador; is not.".to_string(),
        ret: "& is an HTML entity but &ambassador; is not.".to_string(),
      },
      Suite {
        text: "and I quote: &quot;...&quot;".to_string(),
        ret: "and I quote: \"...\"".to_string(),
      },
      Suite {
        text: "Stay home! Practice on Leetcode :)".to_string(),
        ret: "Stay home! Practice on Leetcode :)".to_string(),
      },
      Suite {
        text: "x &gt; y &amp;&amp; x &lt; y is always false".to_string(),
        ret: "x > y && x < y is always false".to_string(),
      },
      Suite {
        text: "leetcode.com&frasl;problemset&frasl;all".to_string(),
        ret: "leetcode.com/problemset/all".to_string(),
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::entity_parser(s.text));
    }
  }
}
