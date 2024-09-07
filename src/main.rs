#![allow(unused_imports)]
#![allow(dead_code)]
mod solution;
use leetcode::{vos, vov, ListNode, TreeNode};
use solution::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(ListNode::from_arr([3]), TreeNode::from_input("[1,5,3,null,4,null,3]"), true)]
    fn test(
        #[case] head: Option<Box<ListNode>>,
        #[case] root: Option<Rc<RefCell<TreeNode>>>,
        #[case] expect: bool,
    ) {
        assert_eq!(solution::Solution::is_sub_path(head, root), expect);
    }
}
