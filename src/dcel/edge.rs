use slotmap::new_key_type;

use super::{half_edge::HalfEdgeId, vertex::VertexId};

new_key_type! {pub struct EdgeId;}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub(super) id: EdgeId,

    pub(super) first: HalfEdgeId,
    pub(super) second: HalfEdgeId,

    pub(super) origin: VertexId,
    pub(super) target: VertexId,
}

impl Edge {
    pub(super) fn new(
        id: EdgeId,
        first: HalfEdgeId,
        second: HalfEdgeId,
        origin: VertexId,
        target: VertexId,
    ) -> Self {
        Self {
            id,
            first,
            second,
            origin,
            target,
        }
    }

    pub fn id(&self) -> EdgeId {
        self.id
    }

    pub fn origin(&self) -> VertexId {
        self.origin
    }

    pub fn target(&self) -> VertexId {
        self.target
    }

    pub fn half_edge(&self) -> HalfEdgeId {
        self.first
    }

    pub fn twin_half_edge(&self) -> HalfEdgeId {
        self.second
    }

    pub(crate) fn next(&self) -> EdgeId {
        todo!()
    }

    pub(crate) fn prev(&self) -> EdgeId {
        todo!()
    }

    pub(crate) fn twin_next(&self) -> EdgeId {
        todo!()
    }

    pub(crate) fn twin_prev(&self) -> EdgeId {
        todo!()
    }
}
