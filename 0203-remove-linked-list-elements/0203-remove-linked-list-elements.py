# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def removeElements(self, head: Optional[ListNode], val: int) -> Optional[ListNode]:
        # handle if head is deleted
        while head and head.val == val:
            head = head.next
        
        # traverse the list
        temp = head
        while temp and temp.next:
            # skip node with val
            if temp.next.val == val:
                temp.next = temp.next.next
            else:
                # not delete
                temp = temp.next
        
        return head
        