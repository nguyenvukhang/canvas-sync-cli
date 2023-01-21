#ifndef FILETREE_H
#define FILETREE_H

#include "types.h"
#include <vector> // for std::vector

using namespace std;

class FileTree {
private:
  // for recursion with the public variant
  void insert_folder(Folder *, string);
  void to_string(string *);

public:
  int id;
  string name;
  vector<FileTree> folders;
  vector<File> files;
  void insert_folder(Folder *);
  void insert_folders(vector<Folder>);
  void insert_file(File *);
  string to_string();
};
void debug(FileTree *);

#endif
