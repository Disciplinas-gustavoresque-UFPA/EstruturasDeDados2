mod critical_path;
mod graph;

pub use critical_path::{CriticalPath, critical_path, reconstruct_path, topological_order};
pub use graph::{Edge, Graph, GraphError};
