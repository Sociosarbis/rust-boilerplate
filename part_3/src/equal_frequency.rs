use super::*;

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut counter = [0; 26];
        for c in word.bytes() {
            counter[(c - 97) as usize] += 1;
        }
        let mut count_group: Vec<(u8, u8)> = Vec::with_capacity(2);
        for count in counter {
            if count != 0 {
                if let Some(index) = count_group.iter().position(|item| item.0 == count) {
                    count_group[index].1 += 1;
                } else {
                    if count_group.len() < 2 {
                        count_group.push((count, 1));
                    } else {
                        return false;
                    }
                }
            }
        }
        if count_group.len() == 2 {
            if count_group[0].0 > count_group[1].0 {
                count_group.swap(0, 1);
            }
            if (count_group[0].0 + 1 == count_group[1].0 && count_group[1].1 == 1)
                || (count_group[0].0 == 1 && count_group[0].1 == 1)
            {
                return true;
            }
        } else if !count_group.is_empty() && (count_group[0].0 == 1 || count_group[0].1 == 1) {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        word: String,
        ret: bool,
    }

    #[test]
    fn test_equal_frequency_simple() {
        let suites = vec![
            Suite {
                word: "abcc".to_string(),
                ret: true,
            },
            Suite {
                word: "aazz".to_string(),
                ret: false,
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::equal_frequency(s.word));
        }
    }
}
