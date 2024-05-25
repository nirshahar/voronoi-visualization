use std::ops::Deref;

use crate::dcel::{
    graph::GeometricGraph,
    vertex::{Vertex, VertexId},
};

pub struct VertexView<'a, V> {
    graph: &'a GeometricGraph<V>,
    vertex: VertexId,
}

impl<'a, V> GeometricGraph<V> {
    pub fn view_vertex(&self, vertex_id: VertexId) -> VertexView<V> {
        VertexView {
            graph: self,
            vertex: vertex_id,
        }
    }
}

impl<'a, V> Deref for VertexView<'a, V> {
    type Target = Vertex<V>;

    fn deref(&self) -> &Self::Target {
        self.graph.vertex(self.vertex)
    }
}

impl<'a, V> From<VertexView<'a, V>> for &'a Vertex<V> {
    fn from(vertex_view: VertexView<'a, V>) -> Self {
        vertex_view.graph.vertex(vertex_view.vertex)
    }
}
