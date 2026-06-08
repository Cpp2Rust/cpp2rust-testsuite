// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstdlib>
#include <iostream>

struct Queue {
  unsigned *elems;
  size_t front;
  size_t back;
  size_t capacity;

  void enqueue(int elem) {
    if (back == capacity) {
      return;
    }
    elems[back++] = elem;
  }

  unsigned dequeue() {
    if (empty()) {
      return -1;
    }
    return elems[front++];
  }

  bool empty() const { return front == back; }
};

struct GraphNode {
  unsigned vertex;
  GraphNode *next;
};

struct Graph {
  const unsigned V;
  GraphNode **adj;

  void push(unsigned src, unsigned dst) {
    adj[src] = new GraphNode{dst, adj[src]};
    adj[dst] = new GraphNode{src, adj[dst]};
  }
};

unsigned *BFS(const Graph &graph, unsigned start_vertex) {
  Queue Q = {new unsigned[graph.V], 0, 0, graph.V};
  bool *visited = new bool[graph.V];
  unsigned *pred = new unsigned[graph.V];
  for (unsigned i = 0; i < graph.V; ++i) {
    visited[i] = false;
    pred[i] = i;
  }

  visited[start_vertex] = true;
  Q.enqueue(start_vertex);
  while (!Q.empty()) {
    int current_vertex = Q.dequeue();
    for (GraphNode *head = graph.adj[current_vertex]; head != nullptr;
         head = head->next) {
      int adj_vertex = head->vertex;
      if (!visited[adj_vertex]) {
        visited[adj_vertex] = true;
        Q.enqueue(adj_vertex);
        pred[adj_vertex] = current_vertex;
      }
    }
  }
  delete[] visited;
  delete[] Q.elems;
  return pred;
}

int main() {
  const size_t N = 300;
  const size_t V = N * N;
  Graph graph = {
      V,
      new GraphNode *[V],
  };

  for (unsigned i = 0; i < V; ++i) {
    graph.adj[i] = nullptr;
  }

  // Build a highly connected dense mesh
  for (unsigned r = 0; r < N; ++r) {
    for (unsigned c = 0; c < N; ++c) {
      unsigned current = r * N + c;

      // Connect to multiple neighbors to the right within radius K
      for (unsigned step = 1; step <= 80; ++step) {
        if (c + step < N) {
          graph.push(current, r * N + (c + step));
        }
      }

      // Connect to multiple neighbors below within radius K
      for (unsigned step = 1; step <= 80; ++step) {
        if (r + step < N) {
          graph.push(current, (r + step) * N + c);
        }
      }
    }
  }

  unsigned *pred = BFS(graph, 0);

  for (unsigned i = 0; i < V; ++i) {
    GraphNode *head = graph.adj[i];
    while (head != nullptr) {
      GraphNode *next = head->next;
      delete head;
      head = next;
    }
  }

  for (unsigned i = 0; i < V; ++i) {
    std::cout << i << " -> " << pred[i] << '\n';
  }

  delete[] graph.adj;
  delete[] pred;

  return 0;
}
