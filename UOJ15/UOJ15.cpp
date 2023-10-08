#include <cstdint>
#include <iostream>
#include <queue>
#include <set>
#include <utility>
auto win = std::set<std::pair<std::int32_t, std::int32_t>>{
    {0, 2}, {0, 3}, {1, 3}, {2, 4}, {3, 4},
    {1, 0}, {2, 1}, {3, 2}, {4, 0}, {4, 1}};

int main() {
        auto A = std::queue<std::int32_t>{};
        auto B = std::queue<std::int32_t>{};
        auto n = std::int32_t{};
        auto na = std::int32_t{};
        auto nb = std::int32_t{};
        auto sa = std::int32_t{};
        auto sb = std::int32_t{};
        std::scanf("%d%d%d", &n, &na, &nb);
        auto tmp = std::int32_t{};
        for (std::size_t i = 0; i < na;
             std::scanf("%d", &tmp), A.push(tmp), i++)
                ;
        for (std::size_t i = 0; i < nb;
             std::scanf("%d", &tmp), B.push(tmp), i++)
                ;
        while (n--) {
                sa += win.count({A.front(), B.front()});
                sb += win.count({B.front(), A.front()});
                A.push(A.front());
                A.pop();
                B.push(B.front());
                B.pop();
        }
        std::cout << sa << " " << sb;
        return 0;
}
