// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iostream>

int main() {
  unsigned long long N = 25000000000;
  unsigned long long sum = 0;
  for (unsigned long long i = 0, j = N; i < j; ++i, --j)
    sum += i + j;
  std::cout << "Sum: " << sum << '\n';
  return 0;
}
