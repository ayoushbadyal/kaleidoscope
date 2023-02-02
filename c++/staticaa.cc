#include "iostream"

using namespace std;

class Entity {
 private:
  // Log(){}
 public:
  float x, y;
  // Entity()=delete;
  Entity() {
    cout << "constructed" << endl;

    x = 0.0f;
    y = 0.0f;
  };
  Entity(int x, int y) {
    x = x;
    y = y;
  };

  ~Entity() { cout << "des" << endl; };
  void print() { cout << x << "&" << y << endl; }
};

void main_(int argc, char const *argv[]) {
  Entity e(1.0f, 4.0f);
  e.print();
}

// destructures

void funv() {
  Entity e;
  e.print();
}

int main(int argc, char const *argv[]) {
  funv();
  return 0;
}
