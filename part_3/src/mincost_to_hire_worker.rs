use super::*;
use std::{collections::BinaryHeap};

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut wage_per_quality: Vec<(usize, f64)> = quality
            .iter()
            .enumerate()
            .map(|(i, &q)| (i, wage[i] as f64 / q as f64))
            .collect();
        wage_per_quality.sort_by(|a, b| { a.1.partial_cmp(&b.1).unwrap() });
        let mut max_heap = BinaryHeap::new();
        let mut sum = 0;
        for i in 0..k as usize {
            let q = quality[wage_per_quality[i].0];
            max_heap.push(q);
            sum += q;
        }
        let mut ret = sum as f64 * wage_per_quality[k as usize - 1].1;
        for &(i, p) in wage_per_quality.iter().skip(k as usize) {
            if let Some(q) = max_heap.pop() {
                sum += quality[i] - q;
                max_heap.push(quality[i]);
                let temp = sum as f64 * p;
                if temp < ret {
                    ret = temp;
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        quality: Vec<i32>,
        wage: Vec<i32>,
        k: i32,
        ret: f64,
    }

    #[test]
    fn test_mincost_to_hire_workers_simple() {
        let suites = vec![
            Suite {
                quality: vec![10, 20, 5],
                wage: vec![70, 50, 30],
                k: 2,
                ret: 105.00000,
            },
            Suite {
                quality: vec![3, 1, 10, 10, 1],
                wage: vec![4, 8, 2, 2, 7],
                k: 3,
                ret: 30.66667,
            },
            Suite {
                quality: vec![3,4,3],
                wage:vec![13,8,20],
                k: 1,
                ret: 8.0
            }
        ];

        for s in suites {
            assert_eq!(
                (s.ret - Solution::mincost_to_hire_workers(s.quality, s.wage, s.k)).abs() < 0.00001,
                true
            );
        }
    }
}
