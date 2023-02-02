
#include <iostream>
#include <string>

using namespace std;

class Entity {
 public:
  virtual string getname() { return "entity"; }
};

class Player : public Entity {
 private:
  string m_name;

 public:
  Player(const string& name) : m_name(name) {}
  string getname() override { return m_name; }
};

void pr(Entity* ent) { cout << ent->getname() << endl; }

int main() {
  Entity* e = new Entity();
  pr(e);
  Player* p = new Player("cherno");
  pr(p);
  Entity* ent = p;
  pr(ent);
}