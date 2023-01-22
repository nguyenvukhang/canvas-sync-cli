#include "canvas_api.h"
#include "errors.h"
#include "httpclient.h"

using json = nlohmann::json;

Profile CanvasApi::profile() {
  return this->get("/api/v1/users/self/profile").get<Profile>();
}

template <typename T> vector<T> to_vec(json j) {
  vector<T> v;
  for (json::iterator it = j.begin(); it != j.end(); ++it) {
    try {
      v.push_back(it->get<T>());
    } catch (json::exception) {
    }
  }
  return v;
}

vector<Course> CanvasApi::courses() {
  json j = this->get("/api/v1/users/self/courses?per_page=118");
  // Simply don't parse invalid courses.
  // These are very real and happen when lecturers/profs want to
  // sandbox some modules and also happen to add you in them.
  return to_vec<Course>(j);
}

vector<File> CanvasApi::course_files(const int *course_id) {
  string url = "/api/v1/courses/";
  url += to_string(*course_id);
  url += "/files?per_page=10000";
  json j = this->get(url.c_str());
  return to_vec<File>(j);
}

vector<Folder> CanvasApi::course_folders(const int *course_id) {
  string url =
      "/api/v1/courses/" + to_string(*course_id) + "/folders?per_page=10000";
  json j = this->get(url.c_str());
  return to_vec<Folder>(j);
}

const char *CanvasApi::get_token_from_env() {
  char *token = std::getenv("CANVAS_TOKEN");
  if (token == NULL) {
    std::cerr << "[error] $CANVAS_TOKEN environment variable not found"
              << std::endl;
  }
  return token;
}

json CanvasApi::get(const char *url) {
  return json::parse(this->cli->get(url));
}
