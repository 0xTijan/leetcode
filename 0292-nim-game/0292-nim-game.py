class Solution:
    def canWinNim(self, n: int) -> bool:
        return n % 4 != 0



"""
WORKING RECURSION SOLUTION (NOT OPTIMAL):

class Solution:
    memo = {}

    def canWinNim(self, n: int) -> bool:
        return self.simulate(n)

    def simulate(self, n: int):
        if n < 4:
            # end recursion
            return True
        
        if n in self.memo:
            return self.memo[n]

        # recursive calls
        a = self.simulate(n-1)  # take 1 stone
        b = self.simulate(n-2)  # take 2 stones
        c = self.simulate(n-3)  # take 3 stones

        self.memo[n-1] = a
        self.memo[n-2] = b
        self.memo[n-3] = c

        # return True only if all paths are winning paths (opponent plays optimally)
        return not (a and b and c)
"""