# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        # check if there even is anything to revers
        if not head:
            return None

        # set the 2 pointers 
        curr = head
        prev = None

        # loop till the end of the list
        while curr:
            nxt = curr.next     # temp variable
            curr.next = prev    # invert the next value in the linked list
            # update pointer
            prev = curr
            curr = nxt
        
        # return the new head (curr = None)
        return prev
            


                