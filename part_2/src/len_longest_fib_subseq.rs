use super::*;
use std::collections::HashMap;


impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::with_capacity(arr.len());
        for &num in &arr {
            map.insert(num, true);
        }
        let mut ret = 0;
        let mut i = 0;
        while i + ret < arr.len() {
            let mut j = i + 1;
            while j < arr.len() && j + ret < arr.len() + 1 {
                let cur = arr[i];
                if map.contains_key(&(arr[j] + cur)) {
                    let diff = arr[j] - cur;
                    if diff >= cur {
                        let mut count = 3;
                        let mut c = cur;
                        let mut n = arr[j];
                        let mut temp = n;
                        n = c + temp;
                        c = temp;
                        while map.contains_key(&(c + n)) {
                            count += 1;
                            temp = n;
                            n = c + temp;
                            c = temp;
                        }
                        if count > ret {
                            ret = count;
                        }
                    }
                }
                j+=1;

            }
            i += 1;
        }
        ret as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        arr: Vec<i32>,
        ret: i32
    }

    #[test]
    fn test_len_longest_fib_subseq() {
        let suites = vec![
            Suite{
                arr: vec![1,2,3,4,5,6,7,8],
                ret: 5
            },
            Suite{
                arr: vec![1,3,7,11,12,14,18],
                ret: 3
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::len_longest_fib_subseq(s.arr));
        }
    }
}