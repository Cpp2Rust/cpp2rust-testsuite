// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iostream>
#include <vector>

void matmul(const std::vector<std::vector<int>> &a,
            const std::vector<std::vector<int>> &b,
            std::vector<std::vector<int>> &c, int n) {
  for (int i = 0; i < n; ++i) {
    for (int k = 0; k < n; ++k) {
      for (int j = 0; j < n; ++j) {
        c[i][j] += a[i][k] * b[k][j];
      }
    }
  }
}

int main() {
  int N = 400;
  std::vector<std::vector<int>> a;
  std::vector<std::vector<int>> b;
  for (int i = 0; i < N; ++i) {
    a.push_back(std::vector<int>(N));
    b.push_back(std::vector<int>(N));
    for (int j = 0; j < N; ++j) {
      a[i][j] = (i + j) % 7;
      b[i][j] = (i * j) % 5;
    }
  }
  long sum = 0;
  for (int r = 0; r < 5; ++r) {
    std::vector<std::vector<int>> c;
    for (int i = 0; i < N; ++i) {
      c.push_back(std::vector<int>(N));
    }
    matmul(a, b, c, N);
    for (int i = 0; i < N; ++i) {
      for (int j = 0; j < N; ++j) {
        sum += c[i][j];
      }
    }
  }
  std::cout << sum << '\n';
  return 0;
}
