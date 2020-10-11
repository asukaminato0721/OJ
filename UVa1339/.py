from collections import Counter


def sol():
    return sorted(Counter(input()).values())


while True:
    try:
        print('YES' if sol() == sol() else 'NO')
    except EOFError:
        break
