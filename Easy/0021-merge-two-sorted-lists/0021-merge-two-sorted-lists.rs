// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut pointer = &mut result;
        
        let mut l1 = list1.clone();
        let mut l2 = list2.clone();

        while let (Some(node1), Some(node2)) = (l1.as_mut(), l2.as_mut()) {
            if node1.val < node2.val {
                pointer.next = l1.clone();
                l1 = l1.unwrap().next;
            } else {
                pointer.next = l2.clone();
                l2 = l2.unwrap().next;
            }
            pointer = pointer.next.as_mut().unwrap();
        }

        if l1.is_some() {
            pointer.next = l1;
        }

        if l2.is_some() {
            pointer.next = l2;
        }

        result.next
    }
}