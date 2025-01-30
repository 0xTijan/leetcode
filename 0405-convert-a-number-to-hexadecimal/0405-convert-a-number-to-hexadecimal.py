class Solution:
    def toHex(self, num: int) -> str:
        if num == 0:
            return "0"

        values = ["a", "b", "c", "d", "e", "f"]

        if num < 0:
            num = 2**32 + num
            
        q = num
        r = 0
        hexString = ""

        while q > 0:
            r = q % 16
            q = q // 16
            print(r, q)
            if r > 9:
                hexString = values[r % 10] + hexString
            else:
                hexString = str(r) + hexString
        
        return hexString
