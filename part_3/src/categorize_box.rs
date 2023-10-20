use super::*;

impl Solution {
  pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
    let mut cate = 0u8;
    if length >= 10000
      || width >= 10000
      || height >= 10000
      || length as i64 * width as i64 * height as i64 >= 1000000000
    {
      cate = 1;
    }

    if mass >= 100 {
      cate |= 2;
    }

    (match cate {
      0 => "Neither",
      1 => "Bulky",
      2 => "Heavy",
      _ => "Both",
    })
    .to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    length: i32,
    width: i32,
    height: i32,
    mass: i32,
    ret: String,
  }

  #[test]
  fn test_categorize_box_simple() {
    let suites = vec![
      Suite {
        length: 1000,
        width: 35,
        height: 700,
        mass: 300,
        ret: "Heavy".to_string(),
      },
      Suite {
        length: 200,
        width: 50,
        height: 800,
        mass: 50,
        ret: "Neither".to_string(),
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::categorize_box(s.length, s.width, s.height, s.mass)
      );
    }
  }
}
