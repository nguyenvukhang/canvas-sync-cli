#ifndef TYPES_H
#define TYPES_H

#include "json.hpp"

using namespace std;
using json = nlohmann::json;

class Profile {
public:
  int id;
  string name;
  string email;
  string login_id;
  string student_id;
};
void to_json(json &j, const Profile &p);
void from_json(const json &j, Profile &p);

void debug();
void debug(Profile *p);

#endif
