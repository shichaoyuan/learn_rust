// Definition for singly-linked list.
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
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_two_numbers_recu(l1, l2, 0)
    }

    fn add_two_numbers_recu(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        mut carry: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            if carry > 0 {
                Some(Box::new(ListNode::new(carry)))
            } else {
                None
            }
        } else {
            Some(Box::new(ListNode {
                next: Self::add_two_numbers_recu(
                    l1.and_then(|x| {
                        carry += x.val;
                        x.next
                    }),
                    l2.and_then(|x| {
                        carry += x.val;
                        x.next
                    }),
                    carry / 10,
                ),
                val: carry % 10,
            }))
        }
    }
}

#[cfg(test)]
mod test {

    use super::{ListNode, Solution};

    #[test]
    fn test() {
        let l1 = build_node_list(&[2, 4, 3]);
        let l2 = build_node_list(&[5, 6, 4]);

        let r = Solution::add_two_numbers(l1, l2);
        assert_eq!(vec![7, 0, 8], to_vec(r));
    }

    fn build_node_list(v: &[i32]) -> Option<Box<ListNode>> {
        if v.is_empty() {
            None
        } else {
            Some(Box::new(ListNode {
                val: v[0],
                next: build_node_list(&v[1..]),
            }))
        }
    }

    fn to_vec(l: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = Vec::new();
        let mut node = l;
        while let Some(a) = node {
            v.push(a.val);
            node = a.next;
        }
        return v;
    }
}
