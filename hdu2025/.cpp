#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
string line;
int main()
{
    while (cin >> line)
    {
        auto sol = max_element(line.begin(), line.end());
        string solution;
        for (auto &&i : line)
        {
            solution += i;
            if (i == *sol)
            {
                solution += "(max)";
            }
        }
        cout << solution << endl;
    }
    return 0;
}
