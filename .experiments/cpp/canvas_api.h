#include "httplib.h"

#include <iostream>

class CanvasApi {
private:
  std::string token;
  std::string base_url = "https://canvas.nus.edu.sg";
  httplib::Result get(const char *url);
  httplib::Client cli();

public:
  CanvasApi(); // use the $CANVAS_TOKEN environment variable
  CanvasApi(const char *token) { this->token = token; };
  void profile();
  void print_token() { std::cerr << this->token << std::endl; }
};
