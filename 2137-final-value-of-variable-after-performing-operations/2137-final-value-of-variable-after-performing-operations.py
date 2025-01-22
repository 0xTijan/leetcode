class Solution:
    def finalValueAfterOperations(self, operations: List[str]) -> int:
        counter = 0
        for i in operations:
            if i[1] == "-":
                counter -= 1
            else:
                counter += 1
        return counter

        