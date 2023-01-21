#include "types.h"

void to_json(json &j, const Profile &p) {
  j = json{{"id", p.id},
           {"name", p.name},
           {"primary_email", p.email},
           {"integration_id", p.student_id}};
}

void from_json(const json &j, Profile &p) {
  j.at("id").get_to(p.id);
  j.at("name").get_to(p.name);
  j.at("primary_email").get_to(p.email);
  j.at("integration_id").get_to(p.student_id);
}
