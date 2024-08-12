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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        /* Method1. Recursive way */
        match (list1, list2) {
            (None, None) => None,
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (Some(mut l1), Some(mut l2)) => {
                if l1.val < l2.val {
                    // Some(Box::new(ListNode {
                    //     val: l1.val,
                    //     next: Self::merge_two_lists(l1.next, Some(l2)),
                    // }))
                    l1.next = Self::merge_two_lists(l1.next, Some(l2));
                    Some(l1)
                } else {
                    // Some(Box::new(ListNode {
                    //     val: l2.val,
                    //     next: Self::merge_two_lists(Some(l1), l2.next),
                    // }))
                    l2.next = Self::merge_two_lists(Some(l1), l2.next);
                    Some(l2)
                }
            }
        }

        /* Method2. Iteration using Pointer */
        // let mut result = ListNode::new(0);
        // let mut pointer = &mut result;

        // let mut l1 = list1.clone();
        // let mut l2 = list2.clone();

        // while let (Some(node1), Some(node2)) = (l1.as_mut(), l2.as_mut()) {
        //     if node1.val < node2.val {
        //         let next_node = node1.next.take();
        //         pointer.next = l1.take();
        //         l1 = next_node;
        //     } else {
        //         let next_node = node2.next.take();
        //         pointer.next = l2.take();
        //         l2 = next_node;
        //     }
        //     pointer = pointer.next.as_mut().unwrap();
        // }

        // if l1.is_some() {
        //     pointer.next = l1;
        // }

        // if l2.is_some() {
        //     pointer.next = l2;
        // }

        // result.next
    }
}

