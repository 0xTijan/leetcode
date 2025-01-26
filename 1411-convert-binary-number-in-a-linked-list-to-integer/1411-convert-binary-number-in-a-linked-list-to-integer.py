# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def getDecimalValue(self, head: Optional[ListNode]) -> int:
        nums = []
        while head:
            nums.append(head.val)
            head = head.next

        # loop through nums from back and do 2**i * num
        decimal = 0
        for index in range(len(nums)-1, -1, -1):
            power = len(nums)-index-1
            decimal += 2**power * nums[index]

        return decimal
