use super::*;

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut ret = vec![];
        for i in left..right + 1 {
            let mut temp = i;
            let mut is_self_dividing = true;
            while temp != 0 {
                let digit = temp % 10;
                if digit == 0 || i % digit != 0 {
                    is_self_dividing = false;
                    break;
                }
                temp /= 10;
            }
            if is_self_dividing {
                ret.push(i);
            }
        }
        return ret;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        left: i32,
        right: i32,
        ret: Vec<i32>
    }

    #[test]
    fn test_self_dividing_numbers_simple() {
        let suites = vec![
            Suite {
                left: 1,
                right: 22,
                ret: vec![1,2,3,4,5,6,7,8,9,11,12,15,22]
            },
            Suite {
                left: 47,
                right: 85,
                ret: vec![48,55,66,77]
            }
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::self_dividing_numbers(s.left, s.right))
        }
    }
}