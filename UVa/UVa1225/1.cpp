#include <iostream>
#include <map>

#define all(x) (x).begin(), (x).end()
std::map<char, int> sol;
int main() {
        int T, n;
        std::cin >> T;
        while (T--) {
                std::cin >> n;
                sol.clear();
                for (auto i = 1; i <= n; i++)
                        for (auto &&j : std::to_string(i))
                                sol[j]++;

                for (auto i = '0'; i < '9'; i++)
                        std::cout << sol[i] << " ";

                std::cout << sol['9'] << std::endl;
        }
        return 0;
}
