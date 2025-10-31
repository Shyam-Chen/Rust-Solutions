struct Solution;

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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true, // 如果兩棵樹都是空的，它們相等
            (Some(p_node), Some(q_node)) => {
                // 如果當前節點的值相等，遞迴比對左子樹和右子樹
                p_node.borrow().val == q_node.borrow().val
                    && Solution::is_same_tree(
                        p_node.borrow().left.clone(),
                        q_node.borrow().left.clone(),
                    )
                    && Solution::is_same_tree(
                        p_node.borrow().right.clone(),
                        q_node.borrow().right.clone(),
                    )
            }
            _ => false, // 其他情況 (如一者為 None，另一者不是)，它們不相等
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_same_tree() {
        {
            let tree1 = Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            })));
            let tree2 = Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            })));
            assert_eq!(Solution::is_same_tree(tree1, tree2), true);
        }

        {
            let tree1 = Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: None,
            })));
            let tree2 = Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            })));
            assert_eq!(Solution::is_same_tree(tree1, tree2), false);
        }

        {
            let tree1 = Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            })));
            let tree2 = Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            })));
            assert_eq!(Solution::is_same_tree(tree1, tree2), false);
        }
    }
}
