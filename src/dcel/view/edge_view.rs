
use impl_tools::autoimpl;

use crate::dcel::{
    edge::{Edge, EdgeId},
    graph::GeometricGraph,
};

use super::{half_edge_view::HalfEdgeView, vertex_view::VertexView};

#[autoimpl(Copy, Clone)]
pub struct EdgeView<'a, V> {
    graph: &'a GeometricGraph<V>,
    edge: EdgeId,
}

impl<V> GeometricGraph<V> {
    pub fn view_edge(&self, edge_id: EdgeId) -> EdgeView<V> {
        EdgeView {
            graph: self,
            edge: edge_id,
        }
    }
}

impl<'a, V> EdgeView<'a, V> {
    pub fn half_edge(self) -> HalfEdgeView<'a, V> {
        self.graph.view_half_edge(self.graph.edge(self.edge).first)
    }

    pub fn twin_edge(self) -> HalfEdgeView<'a, V> {
        self.graph.view_half_edge(self.graph.edge(self.edge).second)
    }

    pub fn next(self) -> EdgeView<'a, V> {
        self.graph.view_edge(self.half_edge().next().full_edge().inner().id) // TODO: why THE FUCK does it not accept it without doing a new `view_edge` on it
    }

    pub fn prev(self) -> EdgeView<'a, V> {
        self.graph.view_edge(self.half_edge().prev().full_edge().inner().id) // TODO: why THE FUCK does it not accept it without doing a new `view_edge` on it
    }

    pub fn twin_next(self) -> EdgeView<'a, V> {
        self.graph
            .view_edge(self.half_edge().twin().next().full_edge().inner().id) // TODO: why THE FUCK does it not accept it without doing a new `view_edge` on it
    }

    pub fn twin_prev(self) -> EdgeView<'a, V> {
        self.graph
            .view_edge(self.half_edge().twin().prev().full_edge().inner().id) // TODO: why THE FUCK does it not accept it without doing a new `view_edge` on it
    }

    pub fn origin(self) -> VertexView<'a, V> {
        self.graph.view_vertex(self.inner().origin)
    }

    pub fn target(self) -> VertexView<'a, V> {
        self.graph.view_vertex(self.inner().target)
    }

    pub fn id(&self) -> EdgeId {
        self.edge
    }

    fn inner(&self) -> &Edge {
        self.graph.edge(self.edge)
    }
}

impl<'a, V> From<EdgeView<'a, V>> for &'a Edge {
    fn from(edge_view: EdgeView<'a, V>) -> Self {
        edge_view.graph.edge(edge_view.edge)
    }
}
