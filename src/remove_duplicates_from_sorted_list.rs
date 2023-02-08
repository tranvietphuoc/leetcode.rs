// https://leetcode.com/problems/remove-duplicates-from-sorted-list/
//

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut h = head;
        let mut ptr = h.as_mut().unwrap();

        while let Some(p) = ptr.next.as_mut() {
            if p.val == ptr.val {
                ptr.next = p.next.take();
            } else {
                ptr = ptr.next.as_mut()?;
            }
        }
        h
    }
}
