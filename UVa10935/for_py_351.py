from collections import deque
while True:
    n = int(input())
    if n == 0:
        break
    elif n == 1:
        print("Discarded cards:\nRemaining card: 1")
    elif n == 2:
        print("Discarded cards: 1\nRemaining card: 2")
    else:
        card = deque(range(1, n+1))
        print("Discarded cards:", end='')
        while len(card) > 2:
            print(' %d,' % card.popleft(), end='')
            card.append(card.popleft())
        print(' {}\nRemaining card: {}'.format(*card))
