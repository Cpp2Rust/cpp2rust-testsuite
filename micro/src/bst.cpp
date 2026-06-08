// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iostream>

struct node_t {
  node_t *left;
  node_t *right;
  int value;
};

node_t *find(node_t *node, int value);

node_t *insert(node_t *node, node_t *new_node);

node_t *find(node_t *node, int value) {
  if (value < node->value && node->left != nullptr) {
    return find(node->left, value);
  } else if (value > node->value && node->right != nullptr) {
    return find(node->right, value);
  } else if (value == node->value) {
    return node;
  }
  return nullptr;
}

node_t *insert(node_t *node, node_t *new_node) {
  if (node == nullptr)
    return new_node;
  if (new_node->value < node->value) {
    node->left = insert(node->left, new_node);
  } else if (new_node->value > node->value) {
    node->right = insert(node->right, new_node);
  }
  return node;
}

int main() {
  const auto N = 25000;
  auto *tree = new node_t{nullptr, nullptr, 0};

  for (int i = 0; i < N; ++i) {
    insert(tree, new node_t{nullptr, nullptr, i});
  }

  for (int i = 0; i < N; ++i) {
    std::cout << "Value: " << i << ", Found: " << find(tree, i)->value << '\n';
  }

  return 0;
}
