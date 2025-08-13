#include <format>
#include <iostream>

struct Rectangle {
  int width;
  int height;

  int area() { return this->width * this->height; }

  bool canHold(Rectangle other) {
    return this->width > other.width && this->height > other.height;
  }

  Rectangle square(int size) { return Rectangle{size, size}; }
};

void main() {
  Rectangle rect1 = Rectangle{
    width : 30,
    height : 50,
  };

  Rectangle rect2 = Rectangle{
    width : 20,
    height : 30,
  };

  std::cout << std::format("Rectangle is {rect1}");

  std::cout << "The area of the rectangle is square pixels" << rect1.area();

  std::cout << "Can hold " + rect2.canHold(rect1) << std::endl;
  println !("Square {:#?}", Rectangle::square(30));
}
