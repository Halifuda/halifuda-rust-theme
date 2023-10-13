#include <iostream>
#include <stdio.h>
#include <inttypes.h>

#define MACRO

class B {};

template <typename T, const int N> class A : public B {
  private:
    T t[N];

  public:
    virtual void f() {}
};

constexpr int t = 10;
static auto a = A<int, t>();

int main(int argc, char *argv[]) {
    uint16_t i = 0;
    printf("Hello World %d\n", i++);
    std::cout << "Hello World " << i++ << std::endl;
    return 0;
}
