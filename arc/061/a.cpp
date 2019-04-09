#include <cmath>
#include <iostream>
#include <string>

int main(int argc, const char *argv[]) {
  int a, b, c, d, e, k;
  std::string no = ":(";
  std::string yes = "Yay!";
  std::cin >> a >> b >> c >> d >> e >> k;
  if (std::abs(a - b) > k) {
    std::cout << no << std::endl;
    return 0;
  }
  if (std::abs(a - c) > k) {
    std::cout << no << std::endl;
    return 0;
  }
  if (std::abs(a - d) > k) {
    std::cout << no << std::endl;
    return 0;
  }
  if (std::abs(a - e) > k) {
    std::cout << no << std::endl;
    return 0;
  }
  if (std::abs(b - c) > k) {
    std::cout << no << std::endl;
    return 0;
  }
  if (std::abs(b - d) > k) {
    std::cout << no << std::endl;
    return 0;
  }
  if (std::abs(b - e) > k) {
    std::cout << no << std::endl;
    return 0;
  }
  if (std::abs(c - d) > k) {
    std::cout << no << std::endl;
    return 0;
  }
  if (std::abs(c - e) > k) {
    std::cout << no << std::endl;
    return 0;
  }
  if (std::abs(d - e) > k) {
    std::cout << no << std::endl;
    return 0;
  }
  std::cout << yes << std::endl;
  return 0;
}
