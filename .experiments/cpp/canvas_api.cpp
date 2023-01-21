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
  Result res = this->get("/api/v1/users/self/profile");
  json j = json::parse(res->body);
  Profile p = j.get<Profile>();
  std::cout << res->body << std::endl;
  return p;
}

vector<Course> CanvasApi::courses() {
  Result res = this->get("/api/v1/users/self/courses?per_page=118");
  json j = json::parse(res->body);
  vector<Course> courses;
  for (json::iterator it = j.begin(); it != j.end(); ++it) {
    try {
      Course c = it->get<Course>();
      courses.push_back(c);
    } catch (json::exception) {
      // simply don't parse invalid courses
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

Client CanvasApi::cli() {
  Client cli(this->base_url);
  cli.set_bearer_token_auth(this->token);
  return cli;
}
