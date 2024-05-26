use slotmap::new_key_type;

use super::{edge::EdgeId, vertex::VertexId};

new_key_type! {pub struct HalfEdgeId;}

#[derive(Debug, Clone, Copy)]
pub struct HalfEdge {
    pub(super) id: HalfEdgeId,

    pub(super) origin: VertexId,
    pub(super) target: VertexId,

    full_edge: Option<EdgeId>,

    pub(crate) twin: HalfEdgeId, // TODO: make `pub(super)` instead of `pub(crate)`

    pub(crate) next: HalfEdgeId, // TODO: make `pub(super)` instead of `pub(crate)`
    pub(super) prev: HalfEdgeId,
}

impl HalfEdge {
    pub(super) fn new(id: HalfEdgeId, origin: VertexId, target: VertexId) -> Self {
        Self {
            id,
            full_edge: None,
            origin,
            target,
            twin: id,
            next: id,
            prev: id,
        }
    }

    pub(super) fn init(
        &mut self,
        full_edge: EdgeId,
        twin_id: HalfEdgeId,
        next_id: HalfEdgeId,
        prev_id: HalfEdgeId,
    ) {
        self.full_edge = Some(full_edge);
        self.twin = twin_id;
        self.next = next_id;
        self.prev = prev_id;
    }

    pub fn id(&self) -> HalfEdgeId {
        self.id
    }

    pub fn origin(&self) -> VertexId {
        self.origin
    }

    pub fn target(&self) -> VertexId {
        self.target
    }

    pub fn full_edge(&self) -> EdgeId {
        self.full_edge.expect("Half-edge is not initialized yet!")
    }
}
