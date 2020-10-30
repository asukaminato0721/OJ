from itertools import zip_longest

while True:
    try:
        n = int(input())
        filename = sorted(input() for _ in range(n))
        字符最大长度 = len(max(filename, key=len))
        每行最多字符 = 60
        cols = (每行最多字符 - 字符最大长度) // (字符最大长度 + 2) + 1
        rows = (n + (cols - 1)) // cols
        print("-" * 60)
        for i, _ in zip(
            zip_longest(
                *(filename[j * rows :] for j in range(n // rows + 1)), fillvalue=""
            ),
            range(rows),
        ):
            for k in i:
                print(k, end=" " * (字符最大长度 - len(k) + 2))
            print()
    except EOFError:
        break
