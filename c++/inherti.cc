#include <iostream>

using namespace std;

class Entity {
 public:
  float x, y;

  void move(float xa, float xb) {
    x += xa;
    y += xb;
  }
};

class player : public Entity {
 public:
  const char* name;
  void printN() { cout << name << endl; }
};

int main(int argc, char const* argv[]) {
  player pl;
  pl.move(4, 4);
  pl.x = 12;
  pl.name = "lamw";
  pl.printN();
  return 0;
}