#include "main.h"
#include "canvas_api.h"
#include "errors.h"
#include "filetree.h"
#include "types.h"
#include <algorithm>
#include <async++.h>
#include <future>
#include <thread>

using json = nlohmann::json;

void version(char *bin_name) {
  std::cout << bin_name << " Version " << canvas_sync_VERSION_MAJOR << "."
            << canvas_sync_VERSION_MINOR << std::endl;
}

int stable() {
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

int smash(int x) { return 1; }

vector<Folder> task(CanvasApi api, Course course) {
  auto res = api.course_folders(&course.id);
  auto tree = new FileTree(&course);
  tree->insert_folders(res);
  debug(tree);
  return res;
}

int main(int argc, char **argv) {
  CanvasApi *api = new CanvasApi();
  vector<Course> courses = api->courses();

  vector<thread> threads;

  for (Course course : courses) {
    thread t(task, *api, course);
    cout << "kick" << course.id << " " << course.name << endl;
    threads.push_back(std::move(t));
  }

  int size = threads.size();
  for (int i = 0; i < size; i++) {
    threads[i].join();
  }

  return 0;
}
