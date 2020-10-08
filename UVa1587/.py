while True:
    try:
        squ = sorted(sorted(int(i) for i in input().split()) for j in range(6))
        if all(squ[0][0] == squ[i][0] for i in (1, 2, 3)) and squ[4][0] == squ[5][0] == squ[0][1] == squ[1][1] and all(squ[2][1] == squ[i][1] for i in (3, 4, 5)):
            print("POSSIBLE")
        else:
            print("IMPOSSIBLE")
    except EOFError:
        break
