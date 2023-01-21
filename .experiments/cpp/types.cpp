#include "types.h"
#include "main.h"

void to_json(json &j, const Course &c) {
  j = json{{"id", c.id}, {"name", c.name}};
}

void from_json(const json &j, Course &c) {
  j.at("id").get_to(c.id);
  j.at("name").get_to(c.name);
}

void eprintln(const char *fmt, ...) {
  va_list args;
  va_start(args, fmt);
  vfprintf(stderr, fmt, args);
  std::cerr << std::endl;
}

void debug() { std::cerr << "Debugging nothing." << std::endl; }
void debug(Profile *p) {
  eprintln("Profile");
  eprintln("* id:         %d", p->id);
  eprintln("* name:       %s", p->name.c_str());
  eprintln("* email:      %s", p->primary_email.c_str());
  eprintln("* login_id:   %s", p->login_id.c_str());
  eprintln("* student_id: %s", p->integration_id.c_str());
}
