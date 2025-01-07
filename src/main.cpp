#include "../include/window.hpp"

int main() {
  Window window("storm", 800, 600);

  while (!window.shouldClose()) {
    window.update();
  }

  return 0;
}
