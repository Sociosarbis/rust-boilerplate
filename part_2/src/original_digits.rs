use super::*;

static ORDER: [usize;10] = [8,4,2,6,0,5,3,1,7,9];
static SYMBOLS: [usize;10] = [6,20,22,23,25,5,7,14,18,4];
static NUMBERS: [[i32;26];10] = [
  [0,0,0,0,1,0,0,0,0,0,0,0,0,0,1,0,0,1,0,0,0,0,0,0,0,1],
  [0,0,0,0,1,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0],
  [0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,1,0,0,0],
  [0,0,0,0,2,0,0,1,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0],
  [0,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,0,1,0,0,1,0,0,0,0,0],
  [0,0,0,0,1,1,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0],
  [0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0],
  [0,0,0,0,2,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,1,0,0,0,0],
  [0,0,0,0,1,0,1,1,1,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0],
  [0,0,0,0,1,0,0,0,1,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0]
];

impl Solution {
  pub fn original_digits(s: String) -> String {
    let mut counter = [0;26];
    for b in s.bytes() {
      counter[b as usize - 97] +=1;
    }
    let mut num_counter = [0;10];
    for i in 0..10 {
      let order = ORDER[i];
      let symbol = SYMBOLS[i];
      if NUMBERS[order][symbol] != 0 && counter[symbol] >= NUMBERS[order][symbol] {
        let count = counter[symbol] / NUMBERS[order][symbol];
        for j in 0..26 {
          if NUMBERS[order][j] != 0 {
            counter[j] -= NUMBERS[order][j] * count;
          }
        }
        num_counter[order] = count;
      }
    }
    let mut ret = String::new();
    for i in 0..10 {
      if num_counter[i] != 0 {
        ret.push_str(i.to_string().repeat(num_counter[i] as usize).as_str());
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
    ret: &'a str
  }

  #[test]
  fn test_original_digits_simple() {
    let suites = vec![
      Suite {
        s: "owoztneoer",
        ret: "012"
      },
      Suite {
        s: "fviefuro",
        ret: "45"
      },
    ];

    for s in suites {
      assert_eq!(s.ret.to_owned(), Solution::original_digits(s.s.to_owned()));
    }
  }
}