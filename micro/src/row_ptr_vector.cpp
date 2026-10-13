// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iostream>
#include <vector>

void smooth(int *p, int n) {
  for (int j = 1; j < n; ++j) {
    p[j] = (p[j] + p[j - 1]) % 1000;
  }
}

long sum(const int *p, const int *end) {
  long s = 0;
  while (p != end) {
    s += *p++;
  }
  return s;
}

int main() {
  int N = 2000;
  std::vector<std::vector<int>> grid;
  for (int i = 0; i < N; ++i) {
    grid.push_back(std::vector<int>(N));
    for (int j = 0; j < N; ++j) {
      grid[i][j] = (i * j) % 7;
    }
  }
  long out = 0;
  for (int r = 0; r < 10; ++r) {
    for (int i = 0; i < N; ++i) {
      int *p = &grid[i][0];
      smooth(p, N);
      out += sum(p, p + N);
    }
  }
  std::cout << out << '\n';
  return 0;
}
