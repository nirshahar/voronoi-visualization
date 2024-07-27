use nannou::geom::Point2;
use slotmap::new_key_type;

use super::half_edge::HalfEdgeId;

new_key_type! {pub struct VertexId;}

pub struct Vertex<Data> {
    pub(super) id: VertexId,

    pub(crate) pos: Point2,
    pub(super) edges: Vec<HalfEdgeId>,
    pub(super) incoming_edges: Vec<HalfEdgeId>,

    pub data: Data,
}

impl<Data> Vertex<Data> {
    pub(super) fn new(id: VertexId, pos: Point2, data: Data) -> Self {
        Self {
            id,
            pos,
            edges: Vec::new(),
            incoming_edges: Vec::new(),
            data,
        }
    }

    pub fn id(&self) -> VertexId {
        self.id
    }

    pub(super) fn clear_edges(&mut self) {
        self.edges.clear();
        self.incoming_edges.clear();
    }
}
