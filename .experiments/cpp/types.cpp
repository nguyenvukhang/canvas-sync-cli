#include "types.h"
#include "main.h"

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
void debug(Course *c) {
  eprintln("Course");
  eprintln("* id:         %d", c->id);
  eprintln("* name:       %s", c->name.c_str());
}
