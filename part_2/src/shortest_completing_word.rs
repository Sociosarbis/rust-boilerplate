use super::*;

impl Solution {
  pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
    let mut mask_list = [0, 0, 0];
    let mut ret: Option<String> = None;
    for c in license_plate.chars() {
      match c {
        'a'..='z'| 'A'..='Z' => {
          Solution::increase_mask_list(&mut mask_list, c.to_ascii_lowercase());
        }
        _ => {}
      }
    }
    
    let mut s_mask_list = [0, 0, 0];
    for s in words {
      if let Some(r) = &ret {
        if r.len() <= s.len() {
          continue;
        }
      }
      for i in 0..3 {
        s_mask_list[i] = 0;
      }
      for c in s.chars() {
        Solution::increase_mask_list(&mut s_mask_list, c);
      }
      let mut is_match = true;
      for i in 0..3 {
        if (s_mask_list[i] & mask_list[i]) != mask_list[i] {
          is_match = false;
          break;
        }
      }
      if is_match {
        ret = Some(s);
      }
    }
    ret.unwrap()
  }

  #[inline]
  fn increase_mask_list(mask_list: &mut [i32], c: char) {
    match c {
      'a'..='j' => {
        mask_list[0] = Solution::increase(mask_list[0], c as i32 - 'a' as i32);
      },
      'k'..='s' => {
        mask_list[1] = Solution::increase(mask_list[1], c as i32 - 'k' as i32);
      },
      't'..='z' => {
        mask_list[2] = Solution::increase(mask_list[2], c as i32 - 't' as i32);
      }
      _ => {}
    }
  }

  #[inline]
  fn increase(mask: i32, i: i32) -> i32 {
    let upper = (i + 1) * 3;
    let lower = i * 3;
    let count = ((mask - (mask >> upper << upper)) >> lower) + 1;
    if count > 7 { mask } else { (count << lower) | mask }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    license_plate: &'a str,
    words: Vec<String>,
    ret: &'a str
  }

  #[test]
  fn test_shortest_completing_word_simple() {
    let suites = vec![
      Suite {
        license_plate: "1s3 PSt",
        words: t1!["step","steps","stripe","stepple"],
        ret: "steps"
      },
      Suite {
        license_plate: "1s3 456",
        words: t1!["looks","pest","stew","show"],
        ret: "pest"
      },
      Suite {
        license_plate: "Ah71752",
        words: t1!["suggest","letter","of","husband","easy","education","drug","prevent","writer","old"],
        ret: "husband"
      },
      Suite {
        license_plate: "OgEu755",
        words: t1!["enough","these","play","wide","wonder","box","arrive","money","tax","thus"],
        ret: "enough"
      },
      Suite {
        license_plate: "iMSlpe4",
        words: t1!["claim","consumer","student","camera","public","never","wonder","simple","thought","use"],
        ret: "simple"
      }
    ];

    for s in suites {
      assert_eq!(s.ret.to_owned(), Solution::shortest_completing_word(s.license_plate.to_owned(), s.words))
    }
  }
}