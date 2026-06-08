// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iostream>
#include <memory>

void initialize(std::unique_ptr<int[]> &array, int N) {
  for (int i = 0; i < N; ++i) {
    array[i] = i;
  }
}

long sum(std::unique_ptr<int[]> *array, int N) {
  long sum = 0;
  for (int i = 0; i < N; ++i) {
    sum += (*array)[i];
  }
  return sum;
}

int main() {
  int N = 100000000;
  long out = 0;
  for (int k = 0; k < 35; ++k) {
    std::unique_ptr<int[]> array = std::make_unique<int[]>(N);
    initialize(array, N);
    out += sum(&array, N);
  }
  std::cout << "Sum: " << out << '\n';
  return 0;
}
