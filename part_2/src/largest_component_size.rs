use super::*;

impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap();
        let mut id_to_group = vec![0;max as usize + 1];
        let mut num_to_group_id = vec![0;max as usize + 1];
        let mut ret = 0;
        for i in 0..nums.len() {
            let mut group_id = Solution::find_root_group_id(&num_to_group_id, nums[i]);
             id_to_group[group_id as usize] += 1;
            for d in 2..((nums[i] as f32).sqrt() as i32 + 1) {
                if nums[i] % d == 0 {
                    let options = [d, nums[i] / d];
                    for &j in &options {
                        let mut other_group_id = Solution::find_root_group_id(&num_to_group_id, j);
                        if group_id != other_group_id {
                            if id_to_group[group_id as usize] < id_to_group[other_group_id as usize] {
                                let temp = other_group_id;
                                other_group_id = group_id;
                                group_id = temp;
                            }
                            id_to_group[group_id as usize] += id_to_group[other_group_id as usize];
                            num_to_group_id[other_group_id as usize] = group_id;
                        }
                    }
                }
            }
        }
        for &v in &id_to_group {
            if v > ret {
                ret = v;
            }
        }
        ret
    }

    fn find_root_group_id(m: &Vec<i32>, mut num: i32) -> i32 {
        while m[num as usize] != 0 {
            num = m[num as usize];
        }
        num
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        nums: Vec<i32>,
        ret: i32
    }

    #[test]
    fn test_largest_component_size_simple() {
        let suites = vec![
            Suite {
                nums: vec![4,6,15,35],
                ret: 4,
            },
            Suite {
                nums: vec![20,50,9,63],
                ret: 2
            },
            Suite {
                nums: vec! [2,3,6,7,4,12,21,39],
                ret: 8
            }
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::largest_component_size(s.nums));
        }
    }
}