n = int(input())
for _ in range(n):
    if _:
        print()
    input()
    line = input()
    for T in range(1, len(line)+1):
        if line == (line*2)[T:T+len(line)]:
            print(T)
            break
