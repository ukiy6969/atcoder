#include <cstdlib>
#include <iostream>
#include <list>
#include <math.h>
#include <string.h>
#include <vector>

int list[10];

int dfs(int end, int i, int bit[10]) {
  if (i >= end) {
    std::cout << "end: " << end << " int: " << i << " list: ";
    for (int j = 0; j < end; j++) {
      std::cout << bit[j] << " ";
    }
    std::cout << std::endl;
    int sum = 0;
    int acc = 0;
    std::cout << " list: ";
    for (int j = 0; j < end; j++) {
      std::cout << bit[j] << " ";
      if (bit[j] == 0) {
        sum += list[j];
      }
      if (bit[j] == 1) {
        acc = list[j] * 10 + list[j + 1];
      }
    }
    std::cout << " sum " << sum << " acc " << acc << std::endl;
    return sum;
  }
  int a = dfs(end, i + 1, bit);
  int list_a[10];
  memcpy(list_a, bit, 10 * sizeof(int));
  list_a[i] = 1;
  int b = dfs(end, i + 1, list_a);
  return a + b;
}

int main() {
  std::cin.tie(0);
  std::ios::sync_with_stdio(false);

  std::string str;
  std::cin >> str;
  for (auto i = 0; i < str.length(); i++) {
    list[i] = (str[i] - '0');
  }
  for (int i = 0; i < str.length(); i++) {
    std::cout << ' ' << list[i];
  }
  std::cout << std::endl;
  int bit[10] = {0};
  int res = dfs(str.length() - 1, 0, bit);
  std::cout << res << std::endl;

  for (int i = 0; i < str.length(); i++) {
    std::cout << ' ' << list[i];
  }
  std::cout << std::endl;
  return EXIT_SUCCESS;
}
