T = int(input())
repeat = set()


def Ducci(li: tuple) -> tuple:
    li += (li[0],)
    return tuple(abs(i-j) for i, j in zip(li, li[1:]))


for i in range(T):
    tmp = int(input())
    line = tuple(int(i) for i in input().split())[:tmp]
    repeat.clear()
    for j in range(1000):
        line = Ducci(line)
        if line in repeat:
            print('LOOP')
            break
        elif all(x == 0 for x in line):
            print('ZERO')
            break
        else:
            repeat.update(line)
    else:
        print('LOOP')
