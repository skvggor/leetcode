s = "abcabcbb"
substring = ""

for i in s:
  for j in s:
    if (i == j):
      print('===>', i)
      substring += j

print(substring)
