def f(still, move):
    return max(len(still), len(move)+min(x for x in range(len(still)+1) if all(i + j <= 3 for i, j in zip(still[x:], move))))


while True:
    try:
        up = list(map(int, input()))
        down = list(map(int, input()))
        print(min(f(up, down), f(down, up)))
    except EOFError:
        break
