use super::*;

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

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let node: Option<Rc<RefCell<TreeNode>>> = None;
        let mut numToNode: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![None; 1001];
        let mut stack = vec![];
        for &num in &nums {
            let mut index = stack.len();
            for i in (0..stack.len()).rev() {
                if stack[i] < num {
                    index = i;
                } else {
                    break;
                }
            }
            if index < stack.len() {
                let mut leftBranch = numToNode[stack[index] as usize].clone();
                let mut temp = leftBranch.clone();
                for j in index + 1..stack.len() {
                    if let Some(n) = temp {
                        n.borrow_mut().right = numToNode[stack[j] as usize].clone();
                        temp = n.borrow().right.clone();
                    }
                }
                let mut subRoot = TreeNode::new(num);
                subRoot.left = leftBranch;
                numToNode[num as usize] = Some(Rc::new(RefCell::new(subRoot)));
                stack.drain(index..stack.len());
            } else {
                numToNode[num as usize] = Some(Rc::new(RefCell::new(TreeNode::new(num)))); 
            }
            stack.push(num);
        }
        if stack.len() == 1 {
            numToNode[stack[0] as usize].clone()
        } else {
            let root = numToNode[stack[0] as usize].clone();
            let mut temp = root.clone();
            for j in 1..stack.len() {
                if let Some(n) = temp {
                    n.borrow_mut().right = numToNode[stack[j] as usize].clone();
                    temp = n.borrow().right.clone();
                }
            }
            root
        }
    }
}
