#ifndef CANVAS_API_H
#define CANVAS_API_H

#include "httplib.h"
#include "types.h"

class CanvasApi {
private:
  std::string token;
  std::string base_url = "https://canvas.nus.edu.sg";
  httplib::Result get(const char *url);
  httplib::Client cli();

public:
  CanvasApi(); // use the $CANVAS_TOKEN environment variable
  CanvasApi(const char *token) { this->token = token; };
  Profile profile();
  void print_token() { std::cerr << this->token << std::endl; }
};

#endif
