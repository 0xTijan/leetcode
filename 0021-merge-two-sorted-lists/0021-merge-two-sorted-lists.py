# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        # create tempNode to simplify the process of appending nodes
        tempNode = ListNode(0)
        tail = tempNode

        # merge lists - check if 1 is bigger then 2 and only increment the list with the smaller number
        while list1 and list2:
            if list1.val <= list2.val:
                tail.next = list1
                list1 = list1.next
            else:
                tail.next = list2
                list2 = list2.next
            tail = tail.next

        # append remaining nodes from either list
        if list1:
            tail.next = list1
        if list2:
            tail.next = list2

        # return the merged list (skipping the tempNode node)
        return tempNode.next
