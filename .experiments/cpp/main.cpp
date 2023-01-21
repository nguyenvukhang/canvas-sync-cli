#include "main.h"
#include "canvas_api.h"
#include "errors.h"
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

int main(int argc, char **argv) {
  CanvasApi *api = new CanvasApi();
  api->profile();
  return 0;
}
