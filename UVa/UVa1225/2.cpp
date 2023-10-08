#include <algorithm>
#include <iostream>
#include <string>

#define all(x) (x).begin(), (x).end()
int main() {
        int T, n;
        std::cin >> T;
        while (T--) {
                std::cin >> n;
                auto str = std::string{};
                for (auto i = 1; i <= n; i++) {
                        str += std::to_string(i);
                }
                for (auto i = '0'; i < '9'; i++) {
                        std::cout << std::count(all(str), i) << " ";
                }
                std::cout << std::count(all(str), '9') << std::endl;
        }
        return 0;
}
