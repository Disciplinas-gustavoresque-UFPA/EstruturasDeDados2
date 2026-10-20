//! Grafo direcionado ponderado armazenado em listas de adjacência.
//!
//! Os vértices são os índices 0..vertex_count. Os pesos são u64
//! (não negativos). Arestas paralelas (mesmo par de vértices) são
//! permitidas; laços (`v -> v`) são rejeitados.

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphError {
    /// Um índice de vértice está fora do intervalo 0..vertex_count.
    VertexOutOfRange {
        vertex: usize,
        vertex_count: usize,
    },
    SelfLoop {
        vertex: usize,
    },
    /// O grafo contém um ciclo, logo não é um DAG.
    CycleDetected,
    /// A soma dos pesos de um caminho excedeu u64::MAX.
    WeightOverflow,
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VertexOutOfRange {
                vertex,
                vertex_count,
            } => {
                write!(f, "vértice {vertex} fora do intervalo 0..{vertex_count}")
            }
            Self::SelfLoop { vertex } => {
                write!(f, "laço no vértice {vertex} não é permitido")
            }
            Self::CycleDetected => write!(f, "o grafo contém um ciclo"),
            Self::WeightOverflow => write!(f, "a soma dos pesos excedeu u64::MAX"),
        }
    }
}

impl Error for GraphError {}

/// Aresta de saída armazenada na lista de adjacência de um vértice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    /// Vértice de destino.
    pub to: usize,
    /// Peso (custo) da aresta.
    pub weight: u64,
}

/// Grafo direcionado ponderado com número fixo de vértices.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Graph {
    adjacency: Vec<Vec<Edge>>,
    in_degrees: Vec<usize>,
    edge_count: usize,
}

impl Graph {
    /// Cria um grafo com vertex_count vértices (0..vertex_count) e sem arestas.
    pub fn new(vertex_count: usize) -> Self {
        Self {
            adjacency: vec![Vec::new(); vertex_count],
            in_degrees: vec![0; vertex_count],
            edge_count: 0,
        }
    }

    /// Número de vértices
    pub fn vertex_count(&self) -> usize {
        self.adjacency.len()
    }

    /// Número de arestas inseridas
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    fn check_vertex(&self, vertex: usize) -> Result<(), GraphError> {
        if vertex < self.adjacency.len() {
            Ok(())
        } else {
            Err(GraphError::VertexOutOfRange {
                vertex,
                vertex_count: self.adjacency.len(),
            })
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, weight: u64) -> Result<(), GraphError> {
        self.check_vertex(from)?;
        self.check_vertex(to)?;
        if from == to {
            return Err(GraphError::SelfLoop { vertex: from });
        }
        self.adjacency[from].push(Edge { to, weight });
        self.in_degrees[to] += 1;
        self.edge_count += 1;
        Ok(())
    }

    /// Grau de entrada de vertex (número de arestas que chegam nele).
    /// [GraphError::VertexOutOfRange] se o vértice não existir.
    pub fn in_degree(&self, vertex: usize) -> Result<usize, GraphError> {
        self.check_vertex(vertex)?;
        Ok(self.in_degrees[vertex])
    }

    /// Grau de saída de vertex (número de arestas que partem dele).
    /// [GraphError::VertexOutOfRange] se o vértice não existir.
    pub fn out_degree(&self, vertex: usize) -> Result<usize, GraphError> {
        self.check_vertex(vertex)?;
        Ok(self.adjacency[vertex].len())
    }

    /// Arestas de saída de `vertex`, na ordem de inserção.
    /// [GraphError::VertexOutOfRange] se o vértice não existir.
    pub fn neighbors(&self, vertex: usize) -> Result<&[Edge], GraphError> {
        self.check_vertex(vertex)?;
        Ok(self.adjacency[vertex].as_slice())
    }

    /// Graus de entrada de todos os vértices, indexados pelo vértice.
    pub fn in_degrees(&self) -> &[usize] {
        &self.in_degrees
    }

    /// Itera sobre todas as arestas como origem, aresta.
    pub fn edges(&self) -> impl Iterator<Item = (usize, Edge)> + '_ {
        self.adjacency
            .iter()
            .enumerate()
            .flat_map(|(from, list)| list.iter().map(move |&edge| (from, edge)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_edge_updates_degrees_and_counts() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1, 4).unwrap();
        graph.add_edge(0, 2, 1).unwrap();
        graph.add_edge(1, 2, 2).unwrap();
        assert_eq!(graph.vertex_count(), 3);
        assert_eq!(graph.edge_count(), 3);
        assert_eq!(graph.out_degree(0).unwrap(), 2);
        assert_eq!(graph.in_degree(2).unwrap(), 2);
        assert_eq!(graph.in_degrees(), &[0, 1, 2]);
        assert_eq!(graph.neighbors(1).unwrap(), &[Edge { to: 2, weight: 2 }]);
        assert_eq!(graph.edges().count(), 3);
    }

    #[test]
    fn test_add_edge_validates_vertices() {
        let mut graph = Graph::new(3);
        assert_eq!(
            graph.add_edge(3, 0, 1),
            Err(GraphError::VertexOutOfRange {
                vertex: 3,
                vertex_count: 3
            })
        );
        assert_eq!(
            graph.add_edge(0, 7, 1),
            Err(GraphError::VertexOutOfRange {
                vertex: 7,
                vertex_count: 3
            })
        );
        assert_eq!(
            graph.add_edge(1, 1, 1),
            Err(GraphError::SelfLoop { vertex: 1 })
        );
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_degree_queries_validate_vertices() {
        let graph = Graph::new(2);
        assert!(graph.in_degree(2).is_err());
        assert!(graph.out_degree(2).is_err());
        assert!(graph.neighbors(2).is_err());
    }
}
