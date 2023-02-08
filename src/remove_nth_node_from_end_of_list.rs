// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

// definition of singly-linked list.
#[derive(ParialEq, Eq, Clone, Debug)]
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut h1 = head.clone();
        let mut h2 = head;
        let mut ptr1 = h1.as_mut()?;
        let mut ptr2 = h2.as_mut()?;
        let mut len = 1;

        // calculate the len of list
        while ptr1.next.is_some() {
            ptr1 = ptr1.next.as_mut()?;
            len += 1;
        }

        // println!("len {}", len);
        // check if n is equal to len, then return the 2nd element of list
        if n == len {
            return ptr2.next.to_owned();
        }

        // else, move to n + 1 element from the end
        for i in 0..(len - n - 1) {
            ptr2 = ptr2.next.as_mut()?;
        }

        // omit the nth element, and assign list to n -1 element from the end of list
        let mut node = ptr2.next.as_mut()?;
        ptr2.next = node.next.take();

        h2
    }
}
