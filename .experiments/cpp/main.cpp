#include "main.h"
#include "canvas_api.h"
#include "errors.h"
#include <vector>

using json = nlohmann::json;

void version(char *bin_name) {
  std::cout << bin_name << " Version " << canvas_sync_VERSION_MAJOR << "."
            << canvas_sync_VERSION_MINOR << std::endl;
}

int main(int argc, char **argv) {
  CanvasApi *api = new CanvasApi();
  Profile profile = api->profile();
  vector<Course> courses = api->courses();

  int course_id = 38518;
  auto list = api->course_files(&course_id);
  for (auto a : list)
    debug(&a);
  debug(&profile);

  return 0;
}
