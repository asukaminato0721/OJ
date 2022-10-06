for _ in range(int(input())):
    _, k = map(int, input().split())
    print(sum(sorted(map(int, input().split()))[-k - 1 :]))
