n = int(input())
for _ in range(n):
    line = input()
    length = len(line)
    print(min((line * 2)[i : i + length] for i in range(length)))
