#include "main.h"
#include "canvas_api.h"
#include "errors.h"
#include "rust.h"

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

void version(char *bin_name) {
  std::cout << bin_name << " Version " << canvas_sync_VERSION_MAJOR << "."
            << canvas_sync_VERSION_MINOR << std::endl;
}

int main(int argc, char **argv) {
  CanvasApi *api = new CanvasApi();
  api->profile();
  return 0;
}
