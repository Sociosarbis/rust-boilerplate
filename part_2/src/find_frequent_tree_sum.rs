use super::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, counter: &mut HashMap<i32, i32>) -> i32 {
        if let Some(node_ref) = node {
            let ret = node_ref.borrow().val
                + Solution::dfs(&node_ref.borrow().left, counter)
                + Solution::dfs(&node_ref.borrow().right, counter);
            if let Some(val) = counter.get_mut(&ret) {
                *val += 1;
            } else {
                counter.insert(ret, 1);
            }
            ret
        } else {
            0
        }
    }

    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut counter = HashMap::new();
        Solution::dfs(&root, &mut counter);
        let mut ret = vec![];
        let mut max = 1;
        for (&k, &v) in &counter {
            if v > max {
                ret.clear();
                max = v;
                ret.push(k);
            } else if v == max {
                ret.push(k);
            }
        }
        ret
    }
}
