#include "cstring"
#include "iostream"
#include <ostream>
#include <stdint.h>
#include <string>

using namespace std;

class LOG {
#define Log(x)            \
  cout << x << m << endl; \
  break

 public:
  enum A {
    LOG_LEVEL_ERROR,
    LOG_LEVEL_WARN,
    LOG_LEVEL_INFO,
  };

 private:
  A m_logLevel = LOG_LEVEL_INFO;

 public:
  void set_log_level(A level) { m_logLevel = level; }

  void message(const char *m) {
    switch (m_logLevel) {
    case A::LOG_LEVEL_ERROR:
      Log("[Error] ");
    case A::LOG_LEVEL_WARN:
      Log("[WARNING] ");
    case A::LOG_LEVEL_INFO:
      Log("[INFO] ");
    }
  }
};

int main() {
  LOG log;
  log.set_log_level(log.LOG_LEVEL_INFO);
  log.message("hello");
  return 0;
}