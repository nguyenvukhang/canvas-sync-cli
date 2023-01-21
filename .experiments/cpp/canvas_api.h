#include "httplib.h"

#include <iostream>

class CanvasApi {
private:
  std::string token;
  httplib::Result get(const char *url);
  httplib::Client cli();

public:
  CanvasApi(const char *token);
  void profile();
  void print_token() { std::cerr << this->token << std::endl; }
};
