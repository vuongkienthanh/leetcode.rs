use std::cell::RefCell;
use std::collections::VecDeque;
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

    pub fn from_input(input: &str) -> Option<Rc<RefCell<TreeNode>>> {
        let convert = input
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|ele| ele.parse::<i32>().ok())
            .collect::<Vec<_>>();

        if !convert.is_empty() {
            let root = Rc::new(RefCell::new(TreeNode::new(convert[0].unwrap())));
            let mut queue = VecDeque::from([Rc::clone(&root)]);
            for slc in convert[1..].chunks(2) {
                let cur = queue.pop_front().unwrap();
                if let Some(v) = slc[0] {
                    let left = Rc::new(RefCell::new(TreeNode::new(v)));
                    cur.borrow_mut().left.replace(Rc::clone(&left));
                    queue.push_back(left);
                }
                if let Some(v) = slc[1] {
                    let right = Rc::new(RefCell::new(TreeNode::new(v)));
                    cur.borrow_mut().right.replace(Rc::clone(&right));
                    queue.push_back(right);
                }
            }
            Some(root)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_convert() {
        assert_eq!(
            TreeNode::from_input("[1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]"),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 8,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                        }))),
                    }))),
                    right: None,
                }))),
            })))
        )
    }
}
