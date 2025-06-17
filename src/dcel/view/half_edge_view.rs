use impl_tools::autoimpl;

use crate::dcel::{
    graph::GeometricGraph,
    half_edge::{HalfEdge, HalfEdgeId},
};

use super::{edge_view::EdgeView, vertex_view::VertexView};

#[autoimpl(Clone, Copy)]
pub struct HalfEdgeView<'a, V> {
    graph: &'a GeometricGraph<V>,
    half_edge: HalfEdgeId,
}

impl<V> GeometricGraph<V> {
    pub fn view_half_edge(&self, half_edge_id: HalfEdgeId) -> HalfEdgeView<V> {
        HalfEdgeView {
            graph: self,
            half_edge: half_edge_id,
        }
    }
}

impl<'a, V> HalfEdgeView<'a, V> {
    pub fn next(self) -> HalfEdgeView<'a, V> {
        let half_edge = self.inner().next;
        HalfEdgeView {
            graph: self.graph,
            half_edge,
        }
    }

    pub fn prev(self) -> HalfEdgeView<'a, V> {
        let half_edge = self.inner().prev;
        HalfEdgeView {
            graph: self.graph,
            half_edge,
        }
    }

    pub fn twin(self) -> HalfEdgeView<'a, V> {
        let half_edge = self.inner().twin;
        HalfEdgeView {
            graph: self.graph,
            half_edge,
        }
    }

    pub fn full_edge(self) -> EdgeView<'a, V> {
        let full_edge_id = self.inner().full_edge();
        self.graph.view_edge(full_edge_id)
    }

    pub fn target(self) -> VertexView<'a, V> {
        let vertex_id = self.inner().target;
        self.graph.view_vertex(vertex_id)
    }

    pub fn origin(self) -> VertexView<'a, V> {
        let vertex_id = self.inner().origin;
        self.graph.view_vertex(vertex_id)
    }

    pub fn id(&self) -> HalfEdgeId {
        self.half_edge
    }

    fn inner(&self) -> &HalfEdge {
        self.graph.half_edge(self.half_edge)
    }
}

impl<'a, V> From<HalfEdgeView<'a, V>> for &'a HalfEdge {
    fn from(half_edge_view: HalfEdgeView<'a, V>) -> Self {
        half_edge_view.graph.half_edge(half_edge_view.half_edge)
    }
}
