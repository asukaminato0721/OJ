for _ in range(int(input())):
    input()
    num_list = sorted(map(int, input().split()))
    print("YES" if all(j - i <= 1 for i, j in zip(num_list, num_list[1:])) else "NO")
