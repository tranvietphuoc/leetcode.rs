// https://leetcode.com/problems/add-two-numbers/

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = Box::new(ListNode::new(0));
        let mut ptr = &mut result; // traverse pointer
        let mut sum = 0;
        let mut list1 = l1.as_ref();
        let mut list2 = l2.as_ref();

        while sum != 0 || list1.is_some() || list2.is_some() {
            if let Some(p1) = list1 {
                sum += p1.val;
                list1 = p1.next.as_ref();
            }

            if let Some(p2) = list2 {
                sum += p2.val;
                list2 = p2.next.as_ref();
            }

            // add number to a list
            let (d, m) = (sum / 10, sum % 10);

            ptr.next = Some(Box::new(ListNode::new(m)));
            ptr = ptr.next.as_mut()?;
            sum = d;
        }

        result.next
    }
}
