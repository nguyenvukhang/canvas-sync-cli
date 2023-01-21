#include "main.h"
#include "canvas_api.h"
#include "errors.h"
#include "filetree.h"
#include "types.h"
#include <algorithm>

using json = nlohmann::json;

void version(char *bin_name) {
  std::cout << bin_name << " Version " << canvas_sync_VERSION_MAJOR << "."
            << canvas_sync_VERSION_MINOR << std::endl;
}

int main(int argc, char **argv) {
  CanvasApi *api = new CanvasApi();
  // Profile profile = api->profile();
  // debug(&profile);

  vector<Course> courses = api->courses();

  int course_id = 38518;
  auto folders = api->course_folders(&course_id);

  // vector<Folder> folders;

  FileTree tree;
  tree.id = 0;
  tree.name = "";

  // clang-format off
  Folder f1; f1.id = 1;
  Folder f2; f2.id = 2;
  Folder f3; f3.id = 3;

  f1.full_name = "foo",         f1.name = "foo";
  f2.full_name = "foo/bar",     f2.name = "bar";
  f3.full_name = "foo/bar/baz", f3.name = "baz";
  // clang-format on
  folders.push_back(f1);
  folders.push_back(f2);
  folders.push_back(f3);

  tree.insert_folders(folders);
  debug(&tree);

  return 0;
}
