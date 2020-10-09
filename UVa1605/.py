alpha = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz'

while True:
    try:
        n = int(input())
        print(2, n, n)
        print('\n'.join(alpha[i]*n for i in range(n)), end='\n'*2)
        print('\n'.join([''.join(alpha[i] for i in range(n))]*n))
    except EOFError:
        break
