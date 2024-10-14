# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        # return if no head or linked list is to short
        if not head or not head.next:
            return None

        # set slow and fast pointers
        slow, fast = head, head

        while fast and fast.next:
            # update
            slow = slow.next
            fast = fast.next.next

            if slow == fast:
                # cycle is detected
                slow = head
                # run until slow and fast meet
                while slow != fast:
                    slow = slow.next
                    fast = fast.next
                # slow and fast have met - start of the cycle
                return slow

        return None
