class Solution:
    def check(self, nums: List[int]) -> bool:
        if len(nums) <= 1:
            return True

        dropCounter = 0
        
        for i in range(0, len(nums)-1):
            if nums[i] > nums[i+1]:
                dropCounter += 1
            if dropCounter > 1:
                return False

        if nums[len(nums)-1] > nums[0]:
            dropCounter += 1
        
        if dropCounter > 1:
            return False
        else:
            return True