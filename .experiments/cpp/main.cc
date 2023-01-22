#include "main.h"
#include "canvas_api.h"
#include "errors.h"
#include "filetree.h"
#include "httpjson.h"
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
  string token = CanvasApi::get_token_from_env();
  string base_url = "https://canvas.nus.edu.sg";
  HttpJson *cli = new HttpJson(&token, &base_url);
  CanvasApi *api = new CanvasApi(cli);
  delete (cli);
  Profile profile = api->profile();
  debug(&profile);

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
  string token = CanvasApi::get_token_from_env();
  string base_url = "https://canvas.nus.edu.sg";
  HttpJson *cli = new HttpJson(&token, &base_url);
  CanvasApi *api = new CanvasApi(cli);
  vector<Course> courses = api->courses();

  vector<thread> threads;
  vector<async::task<vector<Folder>>> results;

  for (Course course : courses) {
    thread t(task, *api, course);
    cout << "kick" << course.id << " " << course.name << endl;
    threads.push_back(std::move(t));
    async::task<vector<Folder>> t2 = async::spawn([&]() -> vector<Folder> {
      vector<Folder> folders = api->course_folders(&course.id);
      return folders;
    });
    results.push_back(std::move(t2));
  }
  auto tall = async::when_all(results);
  tall.then([]() -> void {

  });

  vector<FileTree> trees;

  int size = threads.size();
  for (int i = 0; i < size; i++) {
    threads[i].join();
  }

  return 0;
}
