use super::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    val: i32,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            val,
            children: vec![],
        }))
    }
}

impl Solution {
    pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
        let mut map = vec![vec![false; 501]; 501];
        let mut counter = vec![0; 501];
        let mut queue = vec![];
        for p in pairs {
            if counter[p[0] as usize] == 0 {
                queue.push(p[0]);
            }
            counter[p[0] as usize] += 1;
            map[p[0] as usize][p[1] as usize] = true;

            if counter[p[1] as usize] == 0 {
                queue.push(p[1]);
            }
            counter[p[1] as usize] += 1;
            map[p[1] as usize][p[0] as usize] = true;
        }
        queue.sort_unstable_by(|&a, &b| counter[b as usize].cmp(&counter[a as usize]));
        if counter[queue[0] as usize] + 1 < queue.len() {
            return 0;
        }
        let root = Node::new(queue[0]);
        let mut can_swap = false;
        for &num in queue.iter().skip(1) {
            let mut cur = root.clone();
            let mut found = true;
            while found {
                found = false;
                for node in &cur.clone().borrow().children {
                    if map[num as usize][node.borrow().val as usize] {
                        found = true;
                        cur = node.clone();
                        break;
                    }
                }
            }
            if !can_swap
              && counter[num as usize]
                == counter[cur.borrow().val as usize]
            {
                can_swap = true;
            }
            cur.borrow_mut().children.push(Node::new(num));
        }
        if Solution::check_ways_dfs(&root, 0, &counter) == counter[root.borrow().val as usize] + 1 {
            if can_swap {
                return 2;
            }
            return 1;
        }
        0
    }

    fn check_ways_dfs(node: &Rc<RefCell<Node>>, i: usize, counter: &Vec<usize>) -> usize {
        let mut ret = 1;
        for n in &node.borrow().children {
            let res = Solution::check_ways_dfs(n, i + 1, counter);
            if i + res != counter[n.borrow().val as usize] {
                break;
            }
            ret += res;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        pairs: Vec<Vec<i32>>,
        ret: i32,
    }

    #[test]
    fn test_check_ways_simple() {
        let suites = vec![
            Suite {
                pairs: t2_i![[1, 2], [2, 3]],
                ret: 1,
            },
            Suite {
                pairs: t2_i![[1, 2], [2, 3], [1, 3]],
                ret: 2,
            },
            Suite {
                pairs: t2_i![[1, 2], [2, 3], [2, 4], [1, 5]],
                ret: 0,
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::check_ways(s.pairs));
        }
    }
}
