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

  FileTree *root = new FileTree(0, "root");
  for (Course course : courses) {
    FileTree *tree = new FileTree(&course);
    vector<Folder> folders = api->course_folders(&course.id);
    tree->insert_folders(folders);
    root->insert_tree(tree);
  }

  cout << "-------------------------------------" << endl;
  debug(root);

  return 0;
}
