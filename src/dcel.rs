use nannou::prelude::{Point2, Vec2Angle};
use slotmap::{
    basic::{Values, ValuesMut},
    new_key_type, SlotMap,
};

new_key_type! {pub struct VertexId;}
new_key_type! {pub struct HalfEdgeId;}
new_key_type! {pub struct EdgeId;}
new_key_type! {pub struct FaceId;}

pub struct Vertex<Data> {
    id: VertexId,

    pub pos: Point2,
    edges: Vec<HalfEdgeId>,
    incoming_edges: Vec<HalfEdgeId>,

    pub data: Data,
}

impl<Data> Vertex<Data> {
    fn new(id: VertexId, pos: Point2, data: Data) -> Self {
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
}

#[derive(Debug, Clone, Copy)]
pub struct HalfEdge {
    id: HalfEdgeId,

    origin: VertexId,
    target: VertexId,

    pub(crate) twin: HalfEdgeId, // TODO: make private

    pub next: HalfEdgeId, // TODO: make private
    prev: HalfEdgeId,
}

impl HalfEdge {
    fn new(id: HalfEdgeId, origin: VertexId, target: VertexId) -> Self {
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

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    id: EdgeId,

    first: HalfEdgeId,
    second: HalfEdgeId,

    origin: VertexId,
    target: VertexId,
}

impl Edge {
    fn new(
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
}

struct Face {}

pub struct GeometricGraph<VertexData> {
    vertices: SlotMap<VertexId, Vertex<VertexData>>,
    half_edges: SlotMap<HalfEdgeId, HalfEdge>,
    edges: SlotMap<EdgeId, Edge>,
    faces: SlotMap<FaceId, Face>,
}

impl<VertexData> GeometricGraph<VertexData> {
    pub fn new() -> GeometricGraph<VertexData> {
        Self {
            vertices: SlotMap::with_key(),
            half_edges: SlotMap::with_key(),
            edges: SlotMap::with_key(),
            faces: SlotMap::with_key(),
        }
    }

    pub fn add_vertex(&mut self, pos: Point2, data: VertexData) -> VertexId {
        self.vertices
            .insert_with_key(|id| Vertex::new(id, pos, data))
    }

    pub fn add_edge(&mut self, origin: VertexId, target: VertexId) -> EdgeId {
        // Add the new half-edges and the full edge to the structure
        let edge_id = self
            .half_edges
            .insert_with_key(|edge_id| HalfEdge::new(edge_id, origin, target));
        let twin_id = self
            .half_edges
            .insert_with_key(|twin_id| HalfEdge::new(twin_id, target, origin));

        let full_edge_id = self.edges.insert_with_key(|full_edge_id| {
            Edge::new(full_edge_id, edge_id, twin_id, origin, target)
        });

        // Set the edges as twins of each other
        self.half_edge_mut(edge_id).twin = twin_id;
        self.half_edge_mut(twin_id).twin = edge_id;

        // Find the correct position of the first half-edge
        let first_vertex = self.vertex(origin);
        let second_vertex = self.vertex(target);

        let half_edge_idx = first_vertex
            .edges
            .iter()
            .map(|&other| self.vertex(self.half_edge(other).target).pos - first_vertex.pos)
            .map(|vec| vec.angle())
            .collect::<Vec<f32>>()
            .binary_search_by(|other| {
                other.total_cmp(&(second_vertex.pos - first_vertex.pos).angle())
            })
            .unwrap_or_else(|data| data);

        let incoming_half_edge_idx = second_vertex
            .incoming_edges
            .iter()
            .map(|&other| self.vertex(self.half_edge(other).origin).pos - second_vertex.pos)
            .map(|vec| vec.angle())
            .collect::<Vec<f32>>()
            .binary_search_by(|other| {
                other.total_cmp(&(first_vertex.pos - second_vertex.pos).angle())
            })
            .unwrap_or_else(|data| data);

        let half_twin_idx = second_vertex
            .edges
            .iter()
            .map(|&other| self.vertex(self.half_edge(other).target).pos - second_vertex.pos)
            .map(|vec| vec.angle())
            .collect::<Vec<f32>>()
            .binary_search_by(|other| {
                other.total_cmp(&(first_vertex.pos - second_vertex.pos).angle())
            })
            .unwrap_or_else(|data| data);

        let incoming_twin_idx = first_vertex
            .incoming_edges
            .iter()
            .map(|&other| self.vertex(self.half_edge(other).origin).pos - first_vertex.pos)
            .map(|vec| vec.angle())
            .collect::<Vec<f32>>()
            .binary_search_by(|other| {
                other.total_cmp(&(second_vertex.pos - first_vertex.pos).angle())
            })
            .unwrap_or_else(|data| data);

        // Insert the half-edges into the edge lists in the vertices
        self.vertex_mut(origin).edges.insert(half_edge_idx, edge_id);

        self.vertex_mut(target).edges.insert(half_twin_idx, twin_id);

        self.vertex_mut(target)
            .incoming_edges
            .insert(incoming_half_edge_idx, edge_id);

        self.vertex_mut(origin)
            .incoming_edges
            .insert(incoming_twin_idx, twin_id);

        // Fix the `next` of the new edges and of their previous edges
        let first_vertex = self.vertex(origin);
        let second_vertex = self.vertex(target);
        // Find the `next` of the current edges
        let next_idx = (half_twin_idx + 1) % second_vertex.edges.len();
        let twin_next_idx = (half_edge_idx + 1) % first_vertex.edges.len();

        // Safety: an item was inserted into the vec previously, and the idx is guaranteed to be in bounds.
        let next_id = *second_vertex.edges.get(next_idx).unwrap();
        let twin_next_id = *first_vertex.edges.get(twin_next_idx).unwrap();

        // Find the `prev` of the current edges
        let prev_idx = (incoming_twin_idx + first_vertex.incoming_edges.len() - 1)
            % first_vertex.incoming_edges.len();
        let twin_prev_idx = (incoming_half_edge_idx + second_vertex.incoming_edges.len() - 1)
            % second_vertex.incoming_edges.len();

        // Safety: an item was inserted into the vec previously, and the idx is guaranteed to be in bounds.
        let prev_id = *first_vertex.incoming_edges.get(prev_idx).unwrap();
        let twin_prev_id = *second_vertex.incoming_edges.get(twin_prev_idx).unwrap();

        // Set the `next` of the previous edges
        self.half_edge_mut(prev_id).next = edge_id;
        self.half_edge_mut(twin_prev_id).next = twin_id;

        // Set the `prev` of the next edges
        self.half_edge_mut(next_id).prev = edge_id;
        self.half_edge_mut(twin_next_id).prev = twin_id;

        // Set the `next` of the new edges
        self.half_edge_mut(edge_id).next = next_id;
        self.half_edge_mut(twin_id).next = twin_next_id;

        // Set the `prev` of the new edges
        self.half_edge_mut(edge_id).prev = prev_id;
        self.half_edge_mut(twin_id).prev = twin_prev_id;

        // TODO: set the face correctly

        full_edge_id
    }

    pub fn remove_edge(&mut self, full_edge_id: EdgeId) {
        let full_edge = *self.edge(full_edge_id);

        // Find all `next` and `prev` of the appropriate half-edges
        let edge = self.half_edge(full_edge.first);
        let edge_id = edge.id;
        let edge_next = edge.next;
        let edge_prev = edge.prev;

        let twin_edge = self.half_edge(full_edge.second);
        let twin_id = twin_edge.id;
        let twin_next = twin_edge.next;
        let twin_prev = twin_edge.prev;

        // Fix the `next` and `prev`
        self.half_edge_mut(edge_prev).next = twin_next;
        self.half_edge_mut(edge_next).prev = twin_prev;

        self.half_edge_mut(twin_prev).next = edge_next;
        self.half_edge_mut(twin_next).prev = edge_prev;

        // Remove the half-edges from the outgoing and incoming edges to the vertices
        let first_vertex = self.vertex_mut(full_edge.origin);
        first_vertex.edges.retain(|&id| id != edge_id);
        first_vertex.incoming_edges.retain(|&id| id != twin_id);

        let second_vertex = self.vertex_mut(full_edge.target);
        second_vertex.edges.retain(|&id| id != twin_id);
        second_vertex.incoming_edges.retain(|&id| id != edge_id);

        // Finally, remove the half-edges and the full-edge from the struct
        self.half_edges.remove(full_edge.first);
        self.half_edges.remove(full_edge.second);
        self.edges.remove(full_edge.id);
    }

    pub fn iter_vertices(&self) -> Values<'_, VertexId, Vertex<VertexData>> {
        self.vertices.values()
    }

    pub fn iter_mut_vertices(&mut self) -> ValuesMut<'_, VertexId, Vertex<VertexData>> {
        self.vertices.values_mut()
    }

    pub fn iter_edges(&self) -> Values<'_, EdgeId, Edge> {
        self.edges.values()
    }

    pub fn vertex(&self, vertex_id: VertexId) -> &Vertex<VertexData> {
        self.vertices.get(vertex_id).unwrap()
    }

    pub fn vertex_mut(&mut self, vertex_id: VertexId) -> &mut Vertex<VertexData> {
        self.vertices.get_mut(vertex_id).unwrap()
    }

    pub fn half_edge(&self, edge_id: HalfEdgeId) -> &HalfEdge {
        self.half_edges.get(edge_id).unwrap()
    }

    pub fn half_edge_mut(&mut self, edge_id: HalfEdgeId) -> &mut HalfEdge {
        self.half_edges.get_mut(edge_id).unwrap()
    }

    pub fn edge(&self, edge_id: EdgeId) -> &Edge {
        self.edges.get(edge_id).unwrap()
    }

    pub fn edge_mut(&mut self, edge_id: EdgeId) -> &mut Edge {
        self.edges.get_mut(edge_id).unwrap()
    }

    pub fn origin(&self, edge: &Edge) -> &Vertex<VertexData> {
        self.vertex(edge.origin)
    }

    pub fn target(&self, edge: &Edge) -> &Vertex<VertexData> {
        self.vertex(edge.target)
    }
}

impl<VertexData> Default for GeometricGraph<VertexData> {
    fn default() -> Self {
        Self::new()
    }
}
