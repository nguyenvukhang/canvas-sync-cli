#include "main.h"
#include "errors.h"
#include "httplib.h"
#include "rust.h"

#include <cstdlib>
#include <cstring>
#include <iostream>

struct Folder {
  int id;
  std::string name;
};

Folder *new_folder(int id, const char *name) {
  Folder *f = new Folder;
  f->name = name;
  f->id = id;
  return f;
}

void debug_folder(Folder *folder) {
  printf("Folder (%d: \"%s\")\n", folder->id, folder->name.c_str());
}

char *env_or_throw(const char *env_name) {
  char *token = std::getenv(env_name);
  if (token == NULL) {
    std::cerr << "Tried to get env var: " << env_name << std::endl;
    panic("Environment variable not found");
  }
  return token;
}

void version(char *bin_name) {
  std::cout << bin_name << " Version " << canvas_sync_VERSION_MAJOR << "."
            << canvas_sync_VERSION_MINOR << std::endl;
}

// https://www.nguyenvukhang.com/api/nus

int main(int argc, char **argv) {
  const char *token = env_or_throw("CANVAS_TOKEN");

  Folder *f = new_folder(123, "CS2040");
  debug_folder(f);

  std::cout << "Canvas token: " << token << std::endl;
  return 0;
}
