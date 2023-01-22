#ifndef CANVAS_API_H
#define CANVAS_API_H

#include "httpclient.h"
#include "json.hpp"
#include "types.h"

using namespace std;

class CanvasApi {
private:
  nlohmann::json get(const char *url);
  HttpClient *cli;

public:
  CanvasApi() = delete;
  CanvasApi(HttpClient *cli) { this->cli = std::move(cli); };

  static const char *get_token_from_env();
  Profile profile();
  vector<Course> courses();
  vector<Folder> course_folders(const int *course_id);
  vector<File> course_files(const int *course_id);
};

#endif
