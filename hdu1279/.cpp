#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
#define all(x) x.begin(), x.end()
void sol(int &x)
{
    vector<int> solu;
    bool flag = true;
    while (x != 1)
    {
        if (x & 1)
        {
            flag = false;
            solu.push_back(x);
            x = 3 * x + 1;
        }
        else
        {
            x /= 2;
        }
    }
    if (flag)
    {
        printf("No number can be output !\n");
    }
    else
    {
        for (auto i = solu.begin(); i != solu.end() - 1; i++)
        {
            cout << *i << " ";
        }
        cout << solu.back() << endl;
    }
}
int main()
{
    int n;
    cin >> n;
    while (n--)
    {
        int a;
        cin >> a;
        sol(a);
    }
    return 0;
}
