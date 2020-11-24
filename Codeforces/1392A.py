for _ in range(int(input())):
    n = input()
    print(n if len(set(input().split())) == 1 else 1)
