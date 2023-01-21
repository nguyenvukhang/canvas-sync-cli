#include "filetree.h"
#include <algorithm>
#include <string>

void debug_filetree(FileTree *t, int level) {
  string indent = string(2 * (level + 1), ' ');
  cout << indent << t->id << ", " << t->name << endl;
  for (auto nested : t->folders) {
    debug_filetree(&nested, level + 1);
  }
  for (auto file : t->files) {
    cout << indent << file.filename << endl;
  }
}
void debug(FileTree *t) {
  cerr << "FileTree" << endl;
  debug_filetree(t, 0);
}

FileTree to_tree(Folder *f) {
  FileTree tree;
  tree.id = f->id;
  tree.name = f->name;
  return tree;
}

void FileTree::insert_folder(Folder *f, string state) {
  int slash_idx = state.find('/');

  // no more folders to traverse. insert here.
  if (slash_idx == string::npos) {
    this->folders.push_back(to_tree(f));
    return;
  }

  // find the next folder to go recurse into.
  string query = state.substr(0, slash_idx);
  int size = this->folders.size();
  for (int i = 0; i < size; i++) {
    if (this->folders[i].name != query)
      continue;
    this->folders[i].insert_folder(f, state.substr(slash_idx + 1));
    break;
  }
}

void FileTree::to_string(string *state) {
  *state += "{(" + std::to_string(this->id) + ',' + this->name + "):";
  for (auto nested : this->folders) {
    nested.to_string(state);
  }
  *state += '}';
}

string FileTree::to_string() {
  string state = "";
  this->to_string(&state);
  return state;
}

void FileTree::insert_folder(Folder *f) {
  this->insert_folder(f, f->full_name);
}

bool compareFolderPath(Folder f1, Folder f2) {
  return f1.full_name < f2.full_name;
}

void FileTree::insert_folders(vector<Folder> folders) {
  sort(folders.begin(), folders.end(), compareFolderPath);
  for (auto f : folders)
    this->insert_folder(&f);
}
