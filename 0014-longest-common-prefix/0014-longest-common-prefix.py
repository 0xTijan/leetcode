class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        prefix = ""
        allSame = True
        letterIndex = 0

        while allSame:
            if letterIndex >= len(strs[0]):
                return prefix

            tempPrefix = strs[0][letterIndex]
            for s in strs:
                if letterIndex >= len(s):
                    allSame = False
                elif s[letterIndex] != tempPrefix:
                    allSame = False
            
            if allSame:
                prefix = prefix + tempPrefix
            
            letterIndex += 1


        return prefix 