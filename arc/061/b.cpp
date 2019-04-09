#include <algorithm>
#include <cmath>
#include <iostream>
#include <string>

int r(int list[5], int i, int min, int sum) {
  // std::cout << i << ' ' << min << ' ' << sum << ' ' << list[i] << std::endl;
  if (i == 5) {
    return sum + min;
  }
  if (list[i] % 10 == 0) {
    return r(list, i + 1, min, sum + list[i]);
  }
  if (list[i] % 10 > min % 10) {
    return r(list, i + 1, min, sum + list[i] / 10 * 10 + 10);
  }
  return r(list, i + 1, list[i], sum + min / 10 * 10 + 10);
}

int main(int argc, const char *argv[]) {
  int a, b, c, d, e;
  std::cin >> a >> b >> c >> d >> e;
  int list[5] = {a, b, c, d, e};
  int re = r(list, 1, a, 0);
  std::cout << re << std::endl;
  return 0;
}
