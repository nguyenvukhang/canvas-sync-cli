#define CPPHTTPLIB_OPENSSL_SUPPORT

#include "canvas_api.h"
#include "errors.h"
#include "httplib.h"

#include <cstdlib>
#include <cstring>
#include <iostream>

CanvasApi::CanvasApi(const char *token) {
  printf("copying token: %s\n", token);
  int token_size = strlen(token);
  this->token = "";
  for (int i = 0; i < token_size; i++) {
    this->token += token[i];
  }
}

void CanvasApi::profile() {
  httplib::Client cli("https://canvas.nus.edu.sg");
  cli.set_bearer_token_auth(this->token);
  httplib::Result res = cli.Get("/api/v1/users/self/profile");
  std::cout << res->body << std::endl;
}

httplib::Result CanvasApi::get(const char *url) {
  httplib::Result res = this->cli().Get(url);
  if (res->status == 200) {
    return res;
  }
  auto err = res.error();
  std::cout << "HTTP error: " << httplib::to_string(err) << std::endl;
  panic("Failed network request.");
  return res;
}

httplib::Client CanvasApi::cli() {
  httplib::Client cli("https://canvas.nus.edu.sg");
  cli.set_bearer_token_auth(this->token);
  return cli;
}
