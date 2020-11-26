# A

def foo():
    return map(int, input().split())


for _ in range(int(input())):
    n, m = foo()
    if n == 1:
        print(0)
    elif n == 2:
        print(m)
    else:
        print(2 * m)

# B

from queue import PriorityQueue


def foo():
    return map(int, input().split())


for _ in range(int(input())):
    _, k = foo()
    a, b = PriorityQueue(), PriorityQueue()
    for i in foo():
        a.put(i)
    for i in foo():
        b.put(-i)
    for i in range(k):
        if -b.queue[0] <= a.queue[0]:
            break
        a.put(-b.get())
        b.put(-a.get())
    print(sum(a.queue))
