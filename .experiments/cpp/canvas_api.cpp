#include "canvas_api.h"
#include "errors.h"
#include "main.h"

using namespace httplib;
using json = nlohmann::json;

CanvasApi::CanvasApi() {
  char *token = std::getenv("CANVAS_TOKEN");
  if (token == NULL) {
    panic("$CANVAS_TOKEN environment variable not found");
  }
  this->token = token;
}

Profile CanvasApi::profile() {
  return this->get_json("/api/v1/users/self/profile").get<Profile>();
}

vector<Course> CanvasApi::courses() {
  json j = this->get_json("/api/v1/users/self/courses?per_page=118");
  vector<Course> courses;
  for (json::iterator it = j.begin(); it != j.end(); ++it) {
    try {
      courses.push_back(it->get<Course>());
    } catch (json::exception) {
      // Simply don't parse invalid courses.
      // These are very real and happen when lecturers/profs want to
      // sandbox some modules and also happen to add you in them.
    }
  }
  return courses;
}

Result CanvasApi::get(const char *url) {
  Result res = this->cli().Get(url);
  if (res->status == 200) {
    return res;
  }
  Error err = res.error();
  std::cout << "HTTP error: " << httplib::to_string(err) << std::endl;
  std::cout << "Url used: " << this->base_url << url << std::endl;
  panic("Failed network request.");
  return res;
}

json CanvasApi::get_json(const char *url) {
  return json::parse(this->get(url)->body);
}

Client CanvasApi::cli() {
  Client cli(this->base_url);
  cli.set_bearer_token_auth(this->token);
  return cli;
}
