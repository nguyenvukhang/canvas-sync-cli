#ifndef TYPES_H
#define TYPES_H

#include "json.hpp"
// #include <string>

using namespace std;
using json = nlohmann::json;

struct Profile {
  int id;
  string name;
  string email;
  string email_id;
  string student_id;
};
void to_json(json &j, const Profile &p);
void from_json(const json &j, Profile &p);

#endif
