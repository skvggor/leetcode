class Solution:
    def intToRoman(self, num: int) -> str:
        if (num < 1 or num > 3999):
            return None

        number = ""

        if num < 4:
            return "I" * num

        if num == 4:
            return (num - 3) + "V"

        if num == 5:
            return "V"

        if num > 5 and num < 9:
            value = num - 5
            return "V" + "I" * value

        if num == 9:
            value = num - 8
            return "I" * value + "X"

        if num == 10:
            return "X"


print(Solution().intToRoman(10))
