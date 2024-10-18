# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head:
            return head
        
        prev = head
        node = head.next
        
        while node:
            if prev.val == node.val:
                # delete the duplicate by skipping it
                prev.next = node.next
            else:
                prev = node
            node = node.next
        
        return head
        