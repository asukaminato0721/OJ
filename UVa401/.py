ans = [["not a palindrome.", "a regular palindrome."],
       ["a mirrored string.", "a mirrored palindrome."]]


def pend(p):
    a = "A   3  HIL JM O   2TUVWXY51SE Z  8 "
    if p.isalpha():
        return a[ord(p)-ord('A')]
    else:
        return a[ord(p)-ord('0')+25]


while True:
    try:
        line = input()
        回文 = (line == (line[::-1]))
        镜像 = (line == ''.join(pend(i) for i in line[::-1]))
        print(line, '-- is', ans[镜像][回文], end='\n\n')
    except EOFError:
        break
