memo = {0:0, 1:1, 2:1}

class Solution(object):

    def fib(self, n):
        """
        :type n: int
        :rtype: int
        """
        if n in memo:
            return memo[n]
        else:
            x = self.fib(n-1) + self.fib(n-2)
            memo[n] = x
            return x

        