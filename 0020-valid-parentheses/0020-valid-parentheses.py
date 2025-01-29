class Solution:
    def isValid(self, s: str) -> bool:
        if len(s) % 2 == 1:
            return False
        if s[len(s)-1] == "(" or s[len(s)-1] == "[" or s[len(s)-1] == "{":
            return False
        if s[0] == ")" or s[0] == "]" or s[0] == "}":
            return False

        brackets = []

        for p in s:
            if p == "(" or p == "[" or p == "{":
                brackets.append(p)
            elif len(brackets) == 0:
                return False
            elif p == ")" and brackets[len(brackets)-1] == "(":
                brackets.pop()
            elif p == "]" and brackets[len(brackets)-1] == "[":
                brackets.pop()
            elif p == "}" and brackets[len(brackets)-1] == "{":
                brackets.pop()
            else:
                return False
        
        if len(brackets) is not 0:
            return False

        return True