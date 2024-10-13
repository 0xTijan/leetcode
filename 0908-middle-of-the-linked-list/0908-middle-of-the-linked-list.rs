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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => return None,
            Some(mut h) => {
                let mut counter = 0;
                let mut next_node = Some(h.clone());

                while let Some(node) = next_node {
                    counter += 1;
                    next_node = node.next;
                }

                next_node = Some(h);
                let mut middle: i32 = counter / 2;
                while middle > 0 {
                    if let Some(node) = next_node {
                        next_node = node.next;
                    }
                    middle -= 1;
                }

                next_node
            }
        }
    }
}