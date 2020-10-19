def f(still, move):
    return max(len(still), len(move)+next(x for x in range(200) if all(i + j < 4 for i, j in zip(still[x:], move))))


while True:
    try:
        up = list(map(int, input()))
        down = list(map(int, input()))
        print(min(f(up, down), f(down, up)))
    except EOFError:
        break
