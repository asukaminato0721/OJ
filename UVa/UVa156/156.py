from collections import Counter

文章 = []
while True:
    line = input()
    if line == "#":
        break
    文章 += line.split()
after = Counter(["".join(sorted(i.lower())) for i in 文章])
print(
    *sorted(i for i in 文章 if after["".join(sorted(i.lower()))] == 1), sep="\n"
)
