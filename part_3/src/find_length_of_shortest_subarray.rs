use super::*;

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut j = arr.len();
        while j > 1 {
            if j == arr.len() || arr[j - 1] <= arr[j] {
                j -= 1;
                continue;
            }
            break;
        }
        let mut ret = j as i32;
        for i in 0..n {
            if i == 0 || arr[i] >= arr[i - 1] {
                while j < n {
                    if arr[j] < arr[i] {
                        j += 1;
                    } else {
                        break;
                    }
                }
                let temp = (j - i - 1) as i32;
                if temp < ret {
                    ret = temp;
                    if ret == 0 {
                        break;
                    }
                }
                continue;
            }
            break;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        arr: Vec<i32>,
        ret: i32,
    }

    #[test]
    fn test_find_length_of_shortest_subarray_simple() {
        let suites = vec![
            Suite {
                arr: vec![1, 2, 3, 10, 4, 2, 3, 5],
                ret: 3,
            },
            Suite {
                arr: vec![5, 4, 3, 2, 1],
                ret: 4,
            },
            Suite {
                arr: vec![1, 2, 3],
                ret: 0,
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::find_length_of_shortest_subarray(s.arr));
        }
    }
}
