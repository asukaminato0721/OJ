alpha = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz'

while True:
    try:
        n = int(input())
        print(2, n, n)
        print(*(alpha[i]*n for i in range(n)), sep='\n', end='\n'*2)
        print(*[''.join(alpha[i] for i in range(n))]*n, sep='\n')
    except EOFError:
        break
