class Solution:
    def isArraySpecial(self, nums: List[int]) -> bool:
        if len(nums) < 2:
            return True

        for i in range(0, len(nums)-1):
            if nums[i] % 2 == 1 and nums[i+1] % 2 == 1:
                return False
            elif nums[i] % 2 == 0 and nums[i+1] % 2 == 0:
                return False
        
        return True