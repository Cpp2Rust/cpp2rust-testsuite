// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iostream>

void fib(unsigned long long &n) {
  if (n == 0 || n == 1)
    return;
  unsigned long long n_1 = n - 1;
  unsigned long long n_2 = n - 2;
  fib(n_1);
  fib(n_2);
  n = n_1 + n_2;
}

int main() {
  unsigned long long n = 45;
  fib(n);
  std::cout << "Fibonacci number: " << n << '\n';
  return 0;
}
