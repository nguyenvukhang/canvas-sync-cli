#ifndef TYPES_H
#define TYPES_H

#include "main.h"

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

void debug();
void debug(Profile *p);
void debug(Course *c);

#endif
