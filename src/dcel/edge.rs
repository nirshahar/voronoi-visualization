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
}
