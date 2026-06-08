// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iostream>
#include <memory>

int main() {
  int N = 100000000;
  long sum = 0;
  for (int k = 0; k < 35; ++k) {
    std::unique_ptr<int[]> array = std::make_unique<int[]>(N);
    for (int i = 0; i < N; ++i) {
      array[i] = i;
    }
    for (int i = 0; i < N; ++i) {
      sum += array[i];
    }
  }
  std::cout << sum << '\n';
  return 0;
}
