// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iostream>

const int N = 400;

void matmul(const int (*a)[N], const int (*b)[N], int (*c)[N]) {
  for (int i = 0; i < N; ++i) {
    for (int k = 0; k < N; ++k) {
      for (int j = 0; j < N; ++j) {
        c[i][j] += a[i][k] * b[k][j];
      }
    }
  }
}

int main() {
  int a[N][N];
  int b[N][N];
  int c[N][N];
  for (int i = 0; i < N; ++i) {
    for (int j = 0; j < N; ++j) {
      a[i][j] = (i + j) % 7;
      b[i][j] = (i * j) % 5;
    }
  }
  long sum = 0;
  for (int r = 0; r < 5; ++r) {
    for (int i = 0; i < N; ++i) {
      for (int j = 0; j < N; ++j) {
        c[i][j] = 0;
      }
    }
    matmul(a, b, c);
    for (int i = 0; i < N; ++i) {
      for (int j = 0; j < N; ++j) {
        sum += c[i][j];
      }
    }
  }
  std::cout << sum << '\n';
  return 0;
}
