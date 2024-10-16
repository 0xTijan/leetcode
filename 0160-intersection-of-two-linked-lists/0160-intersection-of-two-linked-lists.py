# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def getIntersectionNode(self, headA: ListNode, headB: ListNode) -> Optional[ListNode]:
        aTemp = headA
        aCounter = 0
        while aTemp and aTemp.next:
            aCounter += 1
            aTemp = aTemp.next

        bTemp = headB
        bCounter = 0
        while bTemp and bTemp.next:
            bCounter += 1
            bTemp = bTemp.next

        # check last nodes in a and b - return if no intersection
        if bTemp != aTemp:
            return None
        
        # set temporary pointers
        firstA = headA
        firstB = headB

        # get nodes the same length from intersection (make them the same lenght from the end)
        if aCounter > bCounter:
            # a list is longer - shorten it
            while aCounter != bCounter:
                firstA = firstA.next
                aCounter -= 1
        else:
            # b list is longer - shorten it
            while aCounter != bCounter:
                firstB = firstB.next
                bCounter -= 1

        # nodes are the same distance from the intersection, loop until the same
        while firstA != firstB:
            firstA = firstA.next
            firstB = firstB.next
        
        # return the intersection node
        return firstA
