#include "../include/window.hpp"

int main() {
    Storm::Window window("storm", 800, 600);
    while (!window.shouldClose()) {
        window.update();
    }
    return 0;
}
