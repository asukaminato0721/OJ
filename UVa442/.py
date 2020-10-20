from collections import deque
li = {a: (int(b), int(c)) for a, b, c in (input().split() for _ in range(int(input())))}


def solve(line):
    stack = deque()
    s = 0
    for i in line:
        if i.isalpha():
            stack.append(li[i])
        elif i == ')':
            (b0, b1), (a0, a1) = stack.pop(), stack.pop()
            if a1 != b0:
                return 'error'
            s += a0*a1*b1
            stack.append((a0, b1))
    return s


while True:
    try:
        print(solve(input()))
    except EOFError:
        break
