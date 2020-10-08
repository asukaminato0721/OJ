n = int(input())
for _ in range(n):
    if _:
        print()
    input()
    line = input()
    for T in range(1, len(line)+1):
        if all(j == k for j, k in zip(line, (line*2)[T:])):
            print(T)
            break
