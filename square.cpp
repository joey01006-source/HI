#include <iostream>

int main() {
    constexpr int side = 5;

    for (int row = 0; row < side; ++row) {
        for (int column = 0; column < side; ++column) {
            std::cout << "* ";
        }
        std::cout << '\n';
    }

    return 0;
}
