#include "main.h"
#include "canvas_api.h"
#include "errors.h"

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
  printf("id -> %s\n", p.student_id.c_str());
  std::cout << res->body << std::endl;
  return p;
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
