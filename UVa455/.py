n = int(input())
for _ in range(n):
    if _:
        print()
    input()
    line = input()*2
    for i in range(1, len(line)+1):
        if all(line[j:j+i] == line[:i] for j in range(i, len(line)-i, i)):
            print(i)
            break
