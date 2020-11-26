n = int(input())
pile = [[i] for i in range(n)]
pos = {i: i for i in range(n)}


def a上面的放回原处(a: int):
    a的位置 = pos[a]
    while pile[a的位置][-1] != a:
        最上面元素 = pile[a的位置].pop()
        pile[最上面元素].append(最上面元素)
        pos[最上面元素] = 最上面元素


def 包括a上面的放到b(a: int, b: int):
    a的位置 = pos[a]
    a的高度 = pile[a的位置].index(a)
    for i in pile[a的位置][a的高度:]:
        pile[pos[b]].append(i)
        pos[i] = pos[b]
    pile[a的位置] = pile[a的位置][:a的高度]


def move_a_onto_b(a: int, b: int):
    a上面的放回原处(a)
    a上面的放回原处(b)
    pile[pos[b]].append(pile[pos[a]].pop())
    pos[a] = pos[b]


def move_a_over_b(a: int, b: int):
    a上面的放回原处(a)
    pile[pos[b]].append(pile[pos[a]].pop())
    pos[a] = pos[b]


def pile_a_onto_b(a: int, b: int):
    a上面的放回原处(b)
    包括a上面的放到b(a, b)


def pile_a_over_b(a: int, b: int):
    包括a上面的放到b(a, b)


while True:
    cmd = input()
    if cmd == "quit":
        break
    cmd1, a, cmd2, b = cmd.split()
    if a == b or pos[int(a)] == pos[int(b)]:
        continue
    {
        ("move", "onto"): move_a_onto_b,
        ("move", "over"): move_a_over_b,
        ("pile", "onto"): pile_a_onto_b,
        ("pile", "over"): pile_a_over_b,
    }[(cmd1, cmd2)](int(a), int(b))
for i, j in enumerate(pile):
    print(f"{i}: " if len(j) > 0 else f"{i}:", end="")
    print(*j, sep=" ")
"""
move a onto b
a和b都是积木的编号，先将a和b上面所有的积木都放回原处，再将a放在b上。
move a over b
a和b都是积木的编号，先将a上面所有的积木放回原处，再将a放在b所在一摞积木的最上面一个积木上。（b上原有积木不动）
pile a onto b
a和b都是积木的编号，将a和其上面所有的积木组成的一摞整体移动到b上。在移动前要先将b上面所有的积木都放回原处。移动的一摞积木要保持原来的顺序不变。
pile a over b
a和b都是积木的编号，将a和其上面所有的积木组成的一摞整体移动到b所在一摞积木的最上面一个积木上。移动的一摞积木要保持原来的顺序不变。
"""
