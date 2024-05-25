use slotmap::new_key_type;

use super::vertex::VertexId;

new_key_type! {pub struct HalfEdgeId;}

#[derive(Debug, Clone, Copy)]
pub struct HalfEdge {
    pub(super) id: HalfEdgeId,

    pub(super) origin: VertexId,
    pub(super) target: VertexId,

    pub(crate) twin: HalfEdgeId, // TODO: make `pub(super)` instead of `pub(crate)`

    pub(crate) next: HalfEdgeId, // TODO: make `pub(super)` instead of `pub(crate)`
    pub(super) prev: HalfEdgeId,
}

impl HalfEdge {
    pub(super) fn new(id: HalfEdgeId, origin: VertexId, target: VertexId) -> Self {
        Self {
            id,
            origin,
            target,
            twin: id,
            next: id,
            prev: id,
        }
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
}
