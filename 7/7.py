class Solution:
    def reverse(self, x: int) -> int:
        if x > 0:
            result = int(str(x)[::-1])
        else:
            result = int(str(x * -1)[::-1]) * -1
        if abs(result) > (2 ** 31 - 1): # overflow case
            return 0
        return result
