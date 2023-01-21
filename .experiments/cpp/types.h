#ifndef TYPES_H
#define TYPES_H

#include "json.hpp"
#include <iostream> // for std::cout and std::cerr in the .cc file

using namespace std;
using json = nlohmann::json;

class Profile {
public:
  int id;
  string name;
  string primary_email;
  string login_id;
  string integration_id;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Profile, id, name, login_id, integration_id,
                                   primary_email);

class Course {
public:
  int id;
  string name;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Course, id, name);

class Folder {
public:
  int id;
  string name;
  string full_name;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Folder, id, name, full_name);

class File {
public:
  int id;
  int folder_id;
  string filename;
  string url;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(File, id, folder_id, filename, url);

void debug(Profile *);
void debug(Course *);
void debug(Folder *);
void debug(File *);

#endif
