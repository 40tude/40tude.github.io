// Open the x64 Native Tools Command Prompt for VS 2022
// cd C:\Users\phili\OneDrive\Documents\40tude.github.io
// launch VSCode from there
// see .vscode\tasks.json for build configuration
// CTRL+SHIFT+B to build
// F5 to debug
// See ./Debug/main.exe

#include <iostream>
#include <string>
#include <vector>

void demo_initialization(void) {

  // Default Initialization
  int x; // x is not initialized
  std::cout << x << std::endl;

  int y{}; // y is initialized to 0

  // Direct Initialization
  int a(10);              // a is initialized to 10
  std::string s("Hello"); // s is initialized to "Hello"

  // Copy Initialization
  int b = 20;              // b is initialized to 20
  std::string t = "World"; // t is initialized to "World"

  // List Initialization, Uniform Initialization
  // To be preferred avoid implicit conversions
  int c{30};                     // c is initialized to 30
  std::vector<int> vec{1, 2, 3}; // vec is initialized to {1, 2, 3}
}

// Copy Constructor
// Happen when an object is created based on another object of the same class
void demo_copy(void) {
  class MyClass {
  public:
    int x;
    MyClass(int val) : x(val) {}                  // default constructor
    MyClass(const MyClass &other) : x(other.x) {} // copy constructor
  };

  MyClass obj1(10);
  MyClass obj2(obj1);  // use the copy constructor
  MyClass obj3 = obj1; // implicit call to the copy constructor
}

// Affectation in french
void demo_assignment(void) {
  class MyClass {
  public:
    int x;
    MyClass(int val) : x(val) {}
    MyClass &operator=(const MyClass &other) {
      if (this != &other) { // avoid self-assignment
        x = other.x;
      }
      return *this;
    }
  };

  MyClass obj1(10);
  MyClass obj2(20);

  obj2 = obj1; // use the assignment (=) operator
}

int main() {
  demo_initialization();
  demo_copy();
  demo_assignment();
}