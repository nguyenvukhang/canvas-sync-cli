#ifndef CANVAS_API_H
#define CANVAS_API_H

#include "main.h"
#include "types.h"

using namespace std;

class CanvasApi {
private:
  string token;
  string base_url = "https://canvas.nus.edu.sg";
  httplib::Result get(const char *url);
  nlohmann::json get_json(const char *url);
  httplib::Client cli();

public:
  CanvasApi(); // use the $CANVAS_TOKEN environment variable
  CanvasApi(const char *token) { this->token = token; };
  Profile profile();
  vector<Course> courses();
  vector<Folder> course_folders(const int *course_id);
  vector<File> course_files(const int *course_id);
  void print_token() { std::cerr << this->token << std::endl; }
};

#endif
