class Solution:
    def tupleSameProduct(self, nums: List[int]) -> int:
        frequency = {}  # product -> frequency ([[x, y]])

        for x in nums:
            for y in nums:
                if x != y:
                    product = x * y
                    if product in frequency: 
                        if [x, y] not in frequency[product] and [y, x] not in frequency[product]:   # filter out for unecessary duplicates
                            frequency[product].append([x, y])
                    else:
                        frequency[product] = [[x, y]]
        
        counter = 0
        for product in frequency:
            l = len(frequency[product])
            if l >= 2:
                # count how many combinations are possible and increment counter: (x^2 - 1x) / 2
                counter += int(((l**2) - l) / 2) * 8
        
        return counter



