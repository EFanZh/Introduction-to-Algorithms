use std::iter::Rev;
use std::slice::IterMut;

pub mod exercises;

fn dfs<W>(graph: &[&[(usize, W)]], vertex: usize, visited: &mut [bool], result: &mut Rev<IterMut<usize>>) {
    if let visited_value @ false = &mut visited[vertex] {
        *visited_value = true;

        for &(next, _) in graph[vertex] {
            dfs(graph, next, visited, result);
        }

        *result.next().unwrap() = vertex;
    }
}

// Topological-Sort(G)
//
// 1  call DFS(G) to compute finishing times v.f for each vertex v
// 2  as each vertex is finished, insert it onto the front of a linked list
// 3  return the linked list of vertices

pub fn topological_sort<W>(g: &[&[(usize, W)]]) -> Vec<usize> {
    let n = g.len();
    let mut visited = vec![false; n];
    let mut result = vec![0; n];
    let mut iter = result.iter_mut().rev();

    for v in 0..n {
        dfs(g, v, &mut visited, &mut iter);
    }

    result
}
