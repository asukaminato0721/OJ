for _ in range(int(input())):
    input()
    _list = list(map(int, input().split()))
    even_cnt = sum(1 for i, j in enumerate(_list) if i % 2 == 0 and j % 2 == 1)
    odd_cnt = sum(1 for i, j in enumerate(_list) if i % 2 == 1 and j % 2 == 0)
    print(even_cnt if even_cnt == odd_cnt else -1)
