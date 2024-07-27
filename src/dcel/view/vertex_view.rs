use impl_tools::autoimpl;
use nannou::glam::Vec2;

use crate::dcel::{
    graph::GeometricGraph,
    vertex::{Vertex, VertexId},
};

use super::half_edge_view::HalfEdgeView;

#[autoimpl(Clone, Copy)]
pub struct VertexView<'a, V> {
    graph: &'a GeometricGraph<V>,
    vertex: VertexId,
}

impl<V> GeometricGraph<V> {
    pub fn view_vertex(&self, vertex_id: VertexId) -> VertexView<V> {
        VertexView {
            graph: self,
            vertex: vertex_id,
        }
    }
}

impl<'a, V> VertexView<'a, V> {
    pub fn find_vertex_to(self, other_vertex: VertexId) -> Option<HalfEdgeView<'a, V>> {
        self.graph
            .vertex(self.vertex)
            .edges
            .iter()
            .find(|&edge_id| self.graph.half_edge(*edge_id).target == other_vertex)
            .map(|&edge_id| self.graph.view_half_edge(edge_id))
    }

    pub fn first_edge(self) -> Option<HalfEdgeView<'a, V>> {
        let edge = *self.inner().edges.first()?;
        Some(self.graph.view_half_edge(edge))
    }

    pub fn pos(&self) -> Vec2 {
        self.inner().pos
    }

    pub fn id(&self) -> VertexId {
        self.vertex
    }

    fn inner(&self) -> &Vertex<V> {
        self.graph.vertex(self.vertex)
    }
}

impl<'a, V> From<VertexView<'a, V>> for &'a Vertex<V> {
    fn from(vertex_view: VertexView<'a, V>) -> Self {
        vertex_view.graph.vertex(vertex_view.vertex)
    }
}
