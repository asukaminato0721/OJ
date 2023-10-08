#include <bits/stdc++.h>
using namespace std;
map<pair<int, int>, int> people;
using T = const decltype(people)::value_type &;
int main()
{
    int n;
    while (scanf("%d", &n) && n)
    {
        int from, to;
        people.clear();
        while (n--)
        {
            scanf("%d%d", &from, &to);
            if (people[{to, from}] > 0)
            {
                people[{to, from}]--;
            }
            else
            {
                people[{from, to}]++;
            }
        }
        if (all_of(people.begin(), people.end(), [](T i) { return i.second == 0; }))
        {
            printf("YES\n");
        }
        else
        {
            printf("NO\n");
        }
    }
    return 0;
}
