class Solution:
    def areAlmostEqual(self, s1: str, s2: str) -> bool:
        if s1 == s2:
            return True  # Already equal, no swap needed

        # Find indices where s1 and s2 differ
        diff_indices = [i for i in range(len(s1)) if s1[i] != s2[i]]

        # There must be exactly two differences for a valid swap
        if len(diff_indices) != 2:
            return False
        
        i, j = diff_indices

        # Check if swapping makes them equal
        return s1[i] == s2[j] and s1[j] == s2[i]
