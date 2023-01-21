#define CPPHTTPLIB_OPENSSL_SUPPORT

#include "canvas_api.h"
#include "errors.h"
#include "httplib.h"

#include <cstdlib>
#include <cstring>
#include <iostream>

CanvasApi::CanvasApi() {
  char *token = std::getenv("CANVAS_TOKEN");
  if (token == NULL) {
    panic("$CANVAS_TOKEN environment variable not found");
  }
  this->token = token;
}

void CanvasApi::profile() {
  httplib::Result res = this->get("/api/v1/users/self/profile");
  std::cout << res->body << std::endl;
}

httplib::Result CanvasApi::get(const char *url) {
  httplib::Result res = this->cli().Get(url);
  if (res->status == 200) {
    return res;
  }
  auto err = res.error();
  std::cout << "HTTP error: " << httplib::to_string(err) << std::endl;
  std::cout << "Url used: " << this->base_url << url << std::endl;
  panic("Failed network request.");
  return res;
}

httplib::Client CanvasApi::cli() {
  httplib::Client cli(this->base_url);
  cli.set_bearer_token_auth(this->token);
  return cli;
}
