#ifndef FILETREE_H
#define FILETREE_H

#include "main.h"
#include "types.h"

using namespace std;

class FileTree {
private:
  void insert_folder(Folder *, string); // for recursion with the public one

public:
  int id;
  string name;
  vector<FileTree> folders;
  vector<File> files;
  void insert_folder(Folder *);
  void insert_folders(vector<Folder>);
  void insert_file(File *);
};
void debug(FileTree *);

#endif
