use std::error::Error;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn from_arr<const A: usize>(v: [i32; A]) -> Option<Box<ListNode>> {
        if v.is_empty() {
            return None;
        }
        let mut root = ListNode { val: 0, next: None };
        let mut cur = &mut root;

        for i in v {
            cur.next = Some(Box::new(ListNode { val: i, next: None }));
            cur = cur.next.as_mut().unwrap();
        }

        root.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_from_arr() {
        assert_eq!(
            ListNode::from_arr([4, 2, 8]),
            Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(8)))
                }))
            }))
        )
    }
}
