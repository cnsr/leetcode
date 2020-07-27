class Solution:
    def romanToInt(self, s: str) -> int:
        roman = {
            "I": 1,
            "V": 5,
            "X": 10,
            "L": 50,
            "C": 100,
            "D": 500,
            "M": 1000,
        }
        res = 0
        for i in range(0, len(s)):
            if s[i] not in ["I", "X", 'C']:
               res += roman[s[i]]
            else:
                try:
                    if s[i] == "I":
                        if s[i+1] in ["V", "X"] or s[i+2] in ["V", "X"]:
                            res -=1
                        else:
                            res += 1
                    elif s[i] == "X":
                        if s[i+1] in  ["L", "C"]:
                            res -= 10
                        else:
                            res += 10
                    elif s[i] == "C":
                        if s[i+1] in ["D", "M"]:
                            res -= 100
                        else:
                            res += 100
                except IndexError:
                    res += roman[s[i]]
        return res
