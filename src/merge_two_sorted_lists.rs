// https://leetcode.com/problems/merge-two-sorted-lists/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0)); // head of list
        let mut ptr = &mut head; // ptr to traverse through list
        let mut l1 = list1.as_ref();
        let mut l2 = list2.as_ref();

        while let (Some(p1), Some(p2)) = (l1, l2) {
            if p1.val < p2.val {
                ptr.next = l1.cloned();
                l1 = p1.next.as_ref();
            } else {
                ptr.next = l2.cloned();
                l2 = p2.next.as_ref()
            }

            ptr = ptr.next.as_mut()?;
        }

        if let Some(p1) = l1 {
            ptr.next = l1.cloned();
        } else if let Some(p2) = l2 {
            ptr.next = l2.cloned();
        }

        head.next // return pointer to the first element of list
    }
}
