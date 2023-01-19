#include "main.h"
#include "rust.h"
#include <iostream>

int main(int argc, char **argv) {
  std::cout << "Hello World!" << std::endl;
  std::cout << add(1, 2) << std::endl;
  std::cout << argv[0] << " Version " << canvas_sync_VERSION_MAJOR << "."
            << canvas_sync_VERSION_MINOR << std::endl;
  return 0;
}
