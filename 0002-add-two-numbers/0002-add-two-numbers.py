# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        n1 = []
        n2 = []

        while l1:
            n1.append(l1.val)
            l1 = l1.next

        while l2:
            n2.append(l2.val)
            l2 = l2.next

        longerList = n1
        shorterList = n2
        if len(n2) > len(n1):
            longerList = n2
            shorterList = n1
        
        q = 0
        s = []
        for i in range(0, len(longerList)):
            shorterValue = 0
            if i < len(shorterList):
                shorterValue = shorterList[i]
            y = q + longerList[i] + shorterValue
            _s = y % 10
            q = y // 10
            s.append(_s)
        
        if q > 0:
            s.append(q)

        # loop through list and append to linked list
        dummyNode = ListNode(0)
        tail = dummyNode
        
        for x in s:
            tail.next = ListNode(x)
            tail = tail.next

        return dummyNode.next
