use super::*;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let n = s.len();
        let nums: Vec<i32> = s.chars().map(|ch| ch as i32 - 48).collect();
        let mut ret: Vec<i32> = nums.clone();
        let mut temp: Vec<i32> = vec![0; n];
        for i in 0..10 {
            for j in (1..n).step_by(2) {
                temp[j] = (nums[j] + a * i) % 10
            }
            for k in 0..(if b & 1 == 1 { 10 } else { 1 }) {
                for j in (0..n).step_by(2) {
                    temp[j] = (nums[j] + a * k) % 10
                }
                let mut max_offset = 0;
                let mut offset = b as usize;
                while offset != 0 {
                    'a: for j in 0..n {
                        let i1 = (max_offset + j) % n;
                        let i2 = (offset + j) % n;
                        if temp[i1] < temp[i2] {
                            break 'a;
                        } else if temp[i1] > temp[i2] {
                            max_offset = offset;
                            break 'a;
                        }
                    }
                    offset = (offset + b as usize) % n;
                }
                for j in 0..n {
                    let i1 = (max_offset + j) % n;
                    if temp[i1] < ret[j] {
                        for j in 0..n {
                            ret[j] = temp[(max_offset + j) % n];
                        }
                        break;
                    } else if temp[i1] > ret[j] {
                        break;
                    }
                }
            }
        }
        ret.into_iter()
            .map(|num| (num + 48) as u8 as char)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        s: String,
        a: i32,
        b: i32,
        ret: String,
    }

    #[test]
    fn test_find_lex_smallest_string_simple() {
        let suites = vec![
            Suite {
                s: "5525".to_string(),
                a: 9,
                b: 2,
                ret: "2050".to_string(),
            },
            Suite {
                s: "74".to_string(),
                a: 5,
                b: 1,
                ret: "24".to_string(),
            },
            Suite {
                s: "0011".to_string(),
                a: 5,
                b: 1,
                ret: "0011".to_string(),
            },
            Suite {
              s: "565510".to_string(),
              a: 7,
              b: 2,
              ret: "105655".to_string()
            }
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::find_lex_smallest_string(s.s, s.a, s.b));
        }
    }
}
