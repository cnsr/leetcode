class Solution:
    def findOcurrences(self, text: str, first: str, second: str) -> List[str]:
        results = []
        words = text.split(" ")
        for index, word in enumerate(words):
            if len(words) > index + 2:
                if word == first and words[index+1] == second:
                    results.append(words[index+2])
        return results
