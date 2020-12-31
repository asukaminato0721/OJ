from fractions import Fraction
while True:
    try:
        k = int(input())
        sol = [(k, (Fraction(1, k)-Fraction(1, y)).denominator, y)
               for y in range(k, 2*k+1)
               if (Fraction(1, k)-Fraction(1, y)).numerator == 1 and (Fraction(1, k)-Fraction(1, y)).denominator >= y]
        print(len(sol))
        for i in sol:
            print('1/%d = 1/%d + 1/%d' % i)
    except EOFError:
        break
