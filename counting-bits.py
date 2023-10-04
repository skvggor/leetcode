# https://leetcode.com/problems/counting-bits/submissions/1066854846/

class Solution:
    def countBits(n: int) -> [int]:
        if n >= 0 and n <= 10**5:
            def decimal_to_binary(decimal: int) -> str:
                if decimal == 0:
                    return "0"

                binary = ""

                while decimal > 0:
                    binary_digit = str(decimal & 1)
                    binary = binary_digit + binary
                    decimal >>= 1

                return binary

            calc = []

            for i in range(0, n + 1):
                binary = decimal_to_binary(i)

                if binary == 0:
                    calc.append(binary)
                else:
                    calc.append(binary.count("1"))

            return calc
        else:
            return None


# Tests
print(Solution.countBits(2))
print(Solution.countBits(5))
print(Solution.countBits(10**6))
print(Solution.countBits(-100))
print(Solution.countBits(100))
