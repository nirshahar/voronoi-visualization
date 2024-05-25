use std::ops::Deref;

use crate::dcel::{
    graph::GeometricGraph,
    half_edge::{HalfEdge, HalfEdgeId},
};

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
    pub fn next(&self) -> HalfEdgeView<V> {
        let half_edge = self.graph.half_edge(self.half_edge).next;
        HalfEdgeView {
            graph: self.graph,
            half_edge,
        }
    }

    pub fn prev(&self) -> HalfEdgeView<V> {
        let half_edge = self.graph.half_edge(self.half_edge).prev;
        HalfEdgeView {
            graph: self.graph,
            half_edge,
        }
    }

    pub fn twin(&self) -> HalfEdgeView<V> {
        let half_edge = self.graph.half_edge(self.half_edge).twin;
        HalfEdgeView {
            graph: self.graph,
            half_edge,
        }
    }

    pub fn id(&self) -> HalfEdgeId {
        self.half_edge
    }
}

impl<'a, V> Deref for HalfEdgeView<'a, V> {
    type Target = HalfEdge;

    fn deref(&self) -> &Self::Target {
        self.graph.half_edge(self.half_edge)
    }
}

impl<'a, V> From<HalfEdgeView<'a, V>> for &'a HalfEdge {
    fn from(half_edge_view: HalfEdgeView<'a, V>) -> Self {
        half_edge_view.graph.half_edge(half_edge_view.half_edge)
    }
}
