class Solution:
    def longestMonotonicSubarray(self, nums: List[int]) -> int:
        increasing = [[nums[0]]]
        decreasing = [[nums[0]]]

        # fill subarrays
        for num in nums:
            iSubarray = increasing[len(increasing)-1]
            if num > iSubarray[len(iSubarray)-1]:
                increasing[len(increasing)-1].append(num)
            else:
                increasing.append([num])
        
            dSubarray = decreasing[len(decreasing)-1]
            if num < dSubarray[len(dSubarray)-1]:
                decreasing[len(decreasing)-1].append(num)
            else:
                decreasing.append([num])

        # check for longest aray in any od subbarray arrays
        maxLeng = 0
        for x in increasing:
            if len(x) > maxLeng:
                maxLeng = len(x)
        for x in decreasing:
            if len(x) > maxLeng:
                maxLeng = len(x)

        return maxLeng
