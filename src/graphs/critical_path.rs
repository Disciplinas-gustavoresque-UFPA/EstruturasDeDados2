//! Caminho crítico (caminho de maior custo) em DAGs ponderados.
//!
//! # Algoritmo
//!
//! 1. Ordenação topológica pelo algoritmo de Kahn (detecta ciclos).
//! 2. Programação dinâmica na ordem topológica: dist[v] é o maior custo de
//!    um caminho que termina em v (iniciando em qualquer vértice), e
//!    pred[v] é o predecessor desse caminho.
//! 3. O vértice final é o de maior dist; o caminho é reconstruído seguindo
//!    pred de trás para frente.
//!
//! # Convenções
//!
//! - Um vértice isolado é um caminho de custo 0 com um único vértice.
//! - Grafo vazio: custo 0 e caminho vazio.
//! - Grafos desconexos: todos os componentes são considerados e vence o
//!   melhor caminho entre eles.
//! - Empates: entre predecessores que dão o mesmo custo, vence o de menor
//!   índice; entre vértices finais de mesmo custo, vence o de menor
//!   índice. O resultado não depende da ordem de inserção das arestas.
//! - Arestas paralelas: vale a de maior peso.
//! - Pesos zero não favorecem caminhos mais longos em número de arestas: o
//!   grafo 0 -> 1 com peso 0 resulta em [0] com custo 0.
//!
//! # Complexidade
//!
//! - Tempo: O(V + E) (Kahn + uma relaxação por aresta + reconstrução O(V)).
//! - Espaço: O(V) adicional (remaining, order, dist, pred), além
//!   do próprio grafo, que ocupa O(V + E).

use std::collections::VecDeque;

use super::graph::{Graph, GraphError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CriticalPath {
    /// Soma dos pesos das arestas.
    pub cost: u64,
    /// Vértices do caminho.
    pub path: Vec<usize>,
}

/// Calcula uma ordenação topológica (algoritmo de Kahn).
/// A fila é FIFO e os vértices iniciais entram em ordem crescente de índice,
/// o que torna a saída determinística.
///
/// # Erros
///
/// [GraphError::CycleDetected] se o grafo tiver um ciclo.
pub fn topological_order(graph: &Graph) -> Result<Vec<usize>, GraphError> {
    let vertex_count = graph.vertex_count();
    let mut remaining: Vec<usize> = graph.in_degrees().to_vec();
    let mut queue: VecDeque<usize> = (0..vertex_count).filter(|&v| remaining[v] == 0).collect();
    let mut order = Vec::with_capacity(vertex_count);

    while let Some(u) = queue.pop_front() {
        order.push(u);
        for edge in graph.neighbors(u)? {
            remaining[edge.to] -= 1;
            if remaining[edge.to] == 0 {
                queue.push_back(edge.to);
            }
        }
    }

    if order.len() == vertex_count {
        Ok(order)
    } else {
        Err(GraphError::CycleDetected)
    }
}

/// Encontra o caminho de maior custo do DAG.
///
/// # Erros
///
/// - [GraphError::CycleDetected] se o grafo não for acíclico;
/// - [GraphError::WeightOverflow] se algum custo acumulado exceder u64::MAX.
pub fn critical_path(graph: &Graph) -> Result<CriticalPath, GraphError> {
    let vertex_count = graph.vertex_count();
    if vertex_count == 0 {
        return Ok(CriticalPath {
            cost: 0,
            path: Vec::new(),
        });
    }

    let order = topological_order(graph)?;
    let mut dist = vec![0u64; vertex_count];
    let mut pred: Vec<Option<usize>> = vec![None; vertex_count];

    for &u in &order {
        let base = dist[u];
        for edge in graph.neighbors(u)? {
            let candidate = base
                .checked_add(edge.weight)
                .ok_or(GraphError::WeightOverflow)?;
            let v = edge.to;
            let wins_tie = match pred[v] {
                None => true,
                Some(current) => u < current,
            };
            if candidate > dist[v] || (candidate == dist[v] && wins_tie) {
                dist[v] = candidate;
                pred[v] = Some(u);
            }
        }
    }

    let end = (0..vertex_count)
        .max_by(|&a, &b| dist[a].cmp(&dist[b]).then(b.cmp(&a)))
        .unwrap_or(0);

    let path = reconstruct_path(&pred, end)?;
    Ok(CriticalPath {
        cost: dist[end],
        path,
    })
}

/// Reconstrói o caminho que termina em `end` seguindo os predecessores.
///
/// # Erros
///
/// - [GraphError::VertexOutOfRange] se `end` ou algum predecessor for inválido;
/// - [GraphError::CycleDetected] se os predecessores formarem um ciclo.
pub fn reconstruct_path(
    predecessors: &[Option<usize>],
    end: usize,
) -> Result<Vec<usize>, GraphError> {
    let vertex_count = predecessors.len();
    if end >= vertex_count {
        return Err(GraphError::VertexOutOfRange {
            vertex: end,
            vertex_count,
        });
    }

    let mut path = vec![end];
    let mut current = end;
    while let Some(previous) = predecessors[current] {
        if previous >= vertex_count {
            return Err(GraphError::VertexOutOfRange {
                vertex: previous,
                vertex_count,
            });
        }
        if path.len() >= vertex_count {
            return Err(GraphError::CycleDetected);
        }
        path.push(previous);
        current = previous;
    }
    path.reverse();
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build(vertices: usize, edges: &[(usize, usize, u64)]) -> Graph {
        let mut graph = Graph::new(vertices);
        for &(from, to, weight) in edges {
            graph.add_edge(from, to, weight).expect("aresta válida");
        }
        graph
    }

    /// Referência independente: DFS exaustiva sobre todos os caminhos.
    fn brute_force_longest(graph: &Graph) -> u64 {
        fn dfs(graph: &Graph, vertex: usize, accumulated: u64, best: &mut u64) {
            *best = (*best).max(accumulated);
            for edge in graph.neighbors(vertex).unwrap() {
                dfs(graph, edge.to, accumulated + edge.weight, best);
            }
        }
        let mut best = 0;
        for vertex in 0..graph.vertex_count() {
            dfs(graph, vertex, 0, &mut best);
        }
        best
    }

    #[test]
    fn test_diamond_graph() {
        let graph = build(4, &[(0, 1, 3), (0, 2, 2), (1, 3, 4), (2, 3, 6)]);
        let result = critical_path(&graph).unwrap();
        assert_eq!(result.cost, 8);
        assert_eq!(result.path, vec![0, 2, 3]);
    }

    #[test]
    fn test_classic_six_vertex_graph() {
        let graph = build(
            6,
            &[
                (0, 1, 5),
                (0, 2, 3),
                (1, 3, 6),
                (1, 2, 2),
                (2, 3, 7),
                (2, 4, 4),
                (3, 4, 1),
                (4, 5, 2),
            ],
        );
        let result = critical_path(&graph).unwrap();
        assert_eq!(result.cost, 17);
        assert_eq!(result.path, vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(result.cost, brute_force_longest(&graph));
    }

    #[test]
    fn test_trace_spans() {
        // 0 = gateway, 1 = auth, 2 = orders, 3 = db, 4 = cache, 5 = resposta
        let graph = build(
            6,
            &[
                (0, 1, 12),
                (0, 2, 5),
                (1, 3, 40),
                (2, 3, 30),
                (2, 4, 8),
                (3, 5, 15),
                (4, 5, 3),
            ],
        );
        let result = critical_path(&graph).unwrap();
        assert_eq!(result.cost, 67);
        assert_eq!(result.path, vec![0, 1, 3, 5]);
        assert_eq!(result.cost, brute_force_longest(&graph));
    }

    #[test]
    fn test_topological_order_respects_edges() {
        let graph = build(4, &[(3, 1, 1), (1, 0, 1), (3, 2, 1), (2, 0, 1)]);
        let order = topological_order(&graph).unwrap();
        let mut position = vec![0; graph.vertex_count()];
        for (index, &vertex) in order.iter().enumerate() {
            position[vertex] = index;
        }
        for (from, edge) in graph.edges() {
            assert!(position[from] < position[edge.to]);
        }
    }

    #[test]
    fn test_empty_graph() {
        let result = critical_path(&Graph::new(0)).unwrap();
        assert_eq!(result.cost, 0);
        assert!(result.path.is_empty());
    }

    #[test]
    fn test_single_vertex() {
        let result = critical_path(&Graph::new(1)).unwrap();
        assert_eq!(result.cost, 0);
        assert_eq!(result.path, vec![0]);
    }

    #[test]
    fn test_isolated_vertices_do_not_hide_the_real_path() {
        let graph = build(4, &[(1, 2, 5)]);
        let result = critical_path(&graph).unwrap();
        assert_eq!(result.cost, 5);
        assert_eq!(result.path, vec![1, 2]);
    }

    #[test]
    fn test_disconnected_components_return_the_best_one() {
        let graph = build(6, &[(0, 1, 3), (2, 3, 4), (3, 4, 1)]);
        let result = critical_path(&graph).unwrap();
        assert_eq!(result.cost, 5);
        assert_eq!(result.path, vec![2, 3, 4]);
    }

    #[test]
    fn test_zero_weight_edges_keep_cost_zero() {
        let graph = build(2, &[(0, 1, 0)]);
        let result = critical_path(&graph).unwrap();
        assert_eq!(result.cost, 0);
        assert_eq!(result.path, vec![0]);
    }

    #[test]
    fn test_parallel_edges_use_the_heaviest() {
        let graph = build(2, &[(0, 1, 2), (0, 1, 9), (0, 1, 4)]);
        let result = critical_path(&graph).unwrap();
        assert_eq!(result.cost, 9);
        assert_eq!(result.path, vec![0, 1]);
    }

    #[test]
    fn test_ties_prefer_smallest_index_regardless_of_insertion_order() {
        let forward = build(4, &[(0, 1, 2), (1, 3, 2), (0, 2, 2), (2, 3, 2)]);
        let backward = build(4, &[(2, 3, 2), (0, 2, 2), (1, 3, 2), (0, 1, 2)]);
        for graph in [&forward, &backward] {
            let result = critical_path(graph).unwrap();
            assert_eq!(result.cost, 4);
            assert_eq!(result.path, vec![0, 1, 3]);
        }

        let tied_ends = build(4, &[(0, 1, 5), (2, 3, 5)]);
        assert_eq!(critical_path(&tied_ends).unwrap().path, vec![0, 1]);
    }

    #[test]
    fn test_cycles_are_detected() {
        let triangle = build(3, &[(0, 1, 1), (1, 2, 1), (2, 0, 1)]);
        assert_eq!(critical_path(&triangle), Err(GraphError::CycleDetected));
        assert_eq!(topological_order(&triangle), Err(GraphError::CycleDetected));

        let tail = build(3, &[(0, 1, 1), (1, 2, 1), (2, 1, 1)]);
        assert_eq!(critical_path(&tail), Err(GraphError::CycleDetected));
    }

    #[test]
    fn test_weight_overflow_is_reported() {
        let graph = build(3, &[(0, 1, u64::MAX), (1, 2, 1)]);
        assert_eq!(critical_path(&graph), Err(GraphError::WeightOverflow));
    }

    #[test]
    fn test_reconstruct_path_works_and_validates() {
        let pred = vec![None, Some(0), Some(1)];
        assert_eq!(reconstruct_path(&pred, 2).unwrap(), vec![0, 1, 2]);
        assert_eq!(reconstruct_path(&pred, 0).unwrap(), vec![0]);
        assert_eq!(
            reconstruct_path(&pred, 3),
            Err(GraphError::VertexOutOfRange {
                vertex: 3,
                vertex_count: 3
            })
        );
        assert_eq!(
            reconstruct_path(&[Some(5)], 0),
            Err(GraphError::VertexOutOfRange {
                vertex: 5,
                vertex_count: 1
            })
        );
        assert_eq!(
            reconstruct_path(&[Some(1), Some(0)], 0),
            Err(GraphError::CycleDetected)
        );
    }
}
