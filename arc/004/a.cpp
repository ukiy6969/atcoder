#include <cstdlib>
#include <iostream>
#include <math.h>
#include <vector>
using namespace std;

int main() {
  std::cin.tie(0);
  std::ios::sync_with_stdio(false);

  int a, b;
  int n;
  double result = 0;
  vector<pair<int, int>> vect;
  std::cin >> n;
  for (int i = 0; i < n; i++) {
    std::cin >> a >> b;
    vect.push_back(make_pair(a, b));
  }
  for (int i = 0; i < n; i++) {
    for (int j = i + 1; j < n; j++) {
      double l = sqrt(pow(vect[i].first - vect[j].first, 2) +
                      pow(vect[i].second - vect[j].second, 2));
      if (result < l) {
        result = l;
      }
    }
  }
  cout << result << endl;
  return EXIT_SUCCESS;
}
