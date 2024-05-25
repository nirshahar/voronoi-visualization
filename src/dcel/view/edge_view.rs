use std::ops::Deref;

use crate::dcel::{
    graph::GeometricGraph,
    edge::{Edge, EdgeId},
};

use super::half_edge_view::HalfEdgeView;

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
    pub fn half_edge(&self) -> HalfEdgeView<V> {
        self.graph
            .view_half_edge(self.graph.edge(self.edge).half_edge())
    }

    pub fn twin_edge(&self) -> HalfEdgeView<V> {
        self.graph
            .view_half_edge(self.graph.edge(self.edge).twin_half_edge())
    }

    pub fn next(&self) -> EdgeView<V> {
        let edge = self.graph.edge(self.edge).next();
        EdgeView {
            graph: self.graph,
            edge,
        }
    }

    pub fn prev(&self) -> EdgeView<V> {
        let edge = self.graph.edge(self.edge).prev();
        EdgeView {
            graph: self.graph,
            edge,
        }
    }

    pub fn twin_next(&self) -> EdgeView<V> {
        let edge = self.graph.edge(self.edge).twin_next();
        EdgeView {
            graph: self.graph,
            edge,
        }
    }

    pub fn twin_prev(&self) -> EdgeView<V> {
        let edge = self.graph.edge(self.edge).twin_prev();
        EdgeView {
            graph: self.graph,
            edge,
        }
    }

    pub fn id(&self) -> EdgeId {
        self.edge
    }
}

impl<'a, V> Deref for EdgeView<'a, V> {
    type Target = Edge;

    fn deref(&self) -> &Self::Target {
        self.graph.edge(self.edge)
    }
}

impl<'a, V> From<EdgeView<'a, V>> for &'a Edge {
    fn from(edge_view: EdgeView<'a, V>) -> Self {
        edge_view.graph.edge(edge_view.edge)
    }
}
