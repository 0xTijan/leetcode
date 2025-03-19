class Solution:
    def search(self, nums: List[int], target: int) -> int:
        # do the binary search
        left = 0
        right = len(nums) - 1
        
        while left <= right:
            middle = (left + right) // 2
            
            if nums[middle] == target:
                return middle
            elif target > nums[middle]:
                left = middle + 1   # +1 since the middle is already checked
            else:
                right = middle -1   # -1 since the middle is already checked

        # no element found return -1
        return -1
        