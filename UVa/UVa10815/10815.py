from sys import stdin

print(
    *sorted(
        set(
            "".join(
                i.lower() if i.isalpha() else " " for i in stdin.read()
            ).split()
        )
    ),
    sep="\n"
)
