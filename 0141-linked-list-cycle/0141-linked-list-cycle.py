# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        # return false if linked list is to short for a cycle
        if not head or not head.next:
            return False
        
        # set slow and fast pointer
        slow = head
        fast = head.next
        
        # run while loop until the slow and fast meet
        # - cycle found => return True
        # - end of the list => return False
        while slow != fast:
            # end of the list
            if not fast or not fast.next:
                return False
            # update the slow and fast pointers
            slow = slow.next
            fast = fast.next.next
        
        return True