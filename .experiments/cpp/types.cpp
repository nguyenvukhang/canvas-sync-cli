#include "types.h"
#include "main.h"

void to_json(json &j, const Profile &p) {
  j = json{{"id", p.id},
           {"name", p.name},
           {"login_id", p.login_id},
           {"primary_email", p.email},
           {"integration_id", p.student_id}};
}

void from_json(const json &j, Profile &p) {
  j.at("id").get_to(p.id);
  j.at("name").get_to(p.name);
  j.at("login_id").get_to(p.login_id);
  j.at("primary_email").get_to(p.email);
  j.at("integration_id").get_to(p.student_id);
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
  eprintln("* email:      %s", p->email.c_str());
  eprintln("* login_id:   %s", p->login_id.c_str());
  eprintln("* student_id: %s", p->student_id.c_str());
}
