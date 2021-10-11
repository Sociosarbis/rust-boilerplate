use super::*;


static UNITS: [&'static str;3]  = ["Thousand", "Million", "Billion"];
static NUMBRES: [&'static str;19] = ["One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"];
static TEN_NUMS: [&'static str;8]  = ["Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];
impl Solution {
  pub fn number_to_words(mut num: i32) -> String {
    if num == 0 {
      return String::from("Zero");
    }
    let mut ret = vec![];
    let mut i = 0;
    while num != 0 {
      let mut cur = num % 1000;
      num /= 1000;
      let mut temp = vec![];
      if cur != 0 {
        let hundreds = cur / 100;
        if hundreds != 0 {
          temp.push(NUMBRES[hundreds as usize - 1]);
          temp.push("Hundred")
        }
        cur %= 100;
        if cur != 0 {
          if cur >= 20 {
            let tens = cur / 10;
            temp.push(TEN_NUMS[tens as usize - 2]);
            cur %= 10;
            if cur != 0 {
              temp.push(NUMBRES[cur as usize - 1]);
            }
          } else {
            temp.push(NUMBRES[cur as usize - 1]);
          }
        }
      }
      if !temp.is_empty() {
        if i > 0 {
          temp.push(UNITS[i - 1]);
        }
        ret.push(temp.join(" "));
      }
      i += 1;
    }
    ret.reverse();
    ret.join(" ")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    num: i32,
    ret: &'a str
  }

  #[test]
  fn test_number_to_words_simple() {
    let suites = vec![
      Suite {
        num: 12345,
        ret: "Twelve Thousand Three Hundred Forty Five"
      },
      Suite {
        num: 1234567,
        ret: "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
      },
      Suite {
        num: 1234567891,
        ret: "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"
      },
    ];

    for s in suites {
      assert_eq!(s.ret.to_owned(), Solution::number_to_words(s.num));
    }
  }
}