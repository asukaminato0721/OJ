#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
deque<int> card;
int n;
int main()
{
    while (scanf("%d", &n) && n)
    {
        if (n == 1)
        {
            printf("Discarded cards:\nRemaining card: 1\n");
            continue;
        }
        else if (n == 2)
        {
            printf("Discarded cards: 1\nRemaining card: 2\n");
            continue;
        }
        else
        {
            card.clear();
            for (size_t i = 1; i <= n; card.push_back(i), i++)
                ;
            printf("Discarded cards:");
            while (card.size() > 2)
            {
                printf(" %d,", card.front());
                card.pop_front();
                card.push_back(card.front());
                card.pop_front();
            }
            printf(" %d\nRemaining card: %d\n", card.front(), card.back());
        }
    }
    return 0;
}
