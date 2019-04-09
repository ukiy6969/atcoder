#include <cstdlib>
#include <iostream>

int main() {
  std::cin.tie(0);
  std::ios::sync_with_stdio(false);

  int k, s, cnt = 0;
  std::cin >> k >> s;

  for (int i = 0; i <= k; i++) {
    for (int j = i; j <= k; j++) {
      for (int l = j; l <= k; l++) {
        if (i + j + l == s) {
          if (i == j && j == l) {
            cnt++;
          } else if (i == j || j == l || i == l) {
            cnt += 3;
          } else {
            cnt += 6;
          }
        }
      }
    }
  }

  std::cout << cnt << std::endl;
  return EXIT_SUCCESS;
}
