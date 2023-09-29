use std::{
    cmp::Ordering,
    slice::{Iter, IterMut},
};

use nannou::prelude::{Point2, Vec2, Vec2Angle};

#[derive(Debug, Clone, Copy)]
pub struct VertexId(usize);

#[derive(Debug, Clone, Copy)]
pub struct HalfEdgeId(pub(crate) usize);

#[derive(Debug, Clone, Copy)]
pub struct EdgeId(usize);

#[derive(Debug, Clone, Copy)]
pub struct FaceId(usize);

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

    fn compare_by_angle<OtherData>(&self, other: &Vertex<OtherData>) -> Ordering {
        // TODO: implement WITHOUT calculating angle
        self.pos.angle().total_cmp(&other.pos.angle())
    }
}

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

pub struct Edge {
    id: EdgeId,

    first: HalfEdgeId,
    second: HalfEdgeId,

    origin: VertexId,
    target: VertexId,
}

impl Edge {
    fn new(id: EdgeId, first: &HalfEdge, second: &HalfEdge) -> Self {
        Self {
            id,
            first: first.id,
            second: second.id,
            origin: first.origin,
            target: first.target,
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
    vertices: Vec<Vertex<VertexData>>,
    half_edges: Vec<HalfEdge>,
    edges: Vec<Edge>,
    faces: Vec<Face>,
}

impl<VertexData> GeometricGraph<VertexData> {
    pub fn new() -> GeometricGraph<VertexData> {
        Self {
            vertices: Vec::new(),
            half_edges: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, pos: Point2, data: VertexData) -> VertexId {
        let id = VertexId(self.vertices.len());
        let vertex = Vertex::new(id, pos, data);

        self.vertices.push(vertex);

        id
    }

    pub fn add_edge(&mut self, first: VertexId, second: VertexId) -> EdgeId {
        let half_edge_id = HalfEdgeId(self.half_edges.len());
        let half_twin_id = HalfEdgeId(self.half_edges.len() + 1);
        let full_edge_id = EdgeId(self.edges.len());

        let mut half_edge = HalfEdge::new(half_edge_id, first, second);
        let mut half_twin = HalfEdge::new(half_twin_id, second, first);
        let full_edge = Edge::new(full_edge_id, &half_edge, &half_twin);

        // Set the edges as twins of each other
        half_edge.twin = half_twin_id;
        half_twin.twin = half_edge_id;

        // Add the edges to the structure
        self.half_edges.push(half_edge);
        self.half_edges.push(half_twin);
        self.edges.push(full_edge);

        // Find the correct position of the first half-edge

        let first_vertex = self.vertex(first);
        let second_vertex = self.vertex(second);

        let half_edge_idx = match first_vertex
            .edges
            .iter()
            .map(|&other| self.vertex(self.half_edge(other).target).pos - first_vertex.pos)
            .collect::<Vec<Vec2>>()
            .binary_search_by(|other| {
                other
                    .angle()
                    .total_cmp(&(second_vertex.pos - first_vertex.pos).angle())
            }) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        let incoming_half_edge_idx = match second_vertex
            .incoming_edges
            .iter()
            .map(|&other| self.vertex(self.half_edge(other).origin).pos - second_vertex.pos)
            .collect::<Vec<Vec2>>()
            .binary_search_by(|other| {
                other
                    .angle()
                    .total_cmp(&(first_vertex.pos - second_vertex.pos).angle())
            }) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        let half_twin_idx = match second_vertex
            .edges
            .iter()
            .map(|&other| self.vertex(self.half_edge(other).target).pos - second_vertex.pos)
            .collect::<Vec<Vec2>>()
            .binary_search_by(|other| {
                other
                    .angle()
                    .total_cmp(&(first_vertex.pos - second_vertex.pos).angle())
            }) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        let incoming_twin_idx = match first_vertex
            .incoming_edges
            .iter()
            .map(|&other| self.vertex(self.half_edge(other).origin).pos - first_vertex.pos)
            .collect::<Vec<Vec2>>()
            .binary_search_by(|other| {
                other
                    .angle()
                    .total_cmp(&(second_vertex.pos - first_vertex.pos).angle())
            }) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        // Insert the half-edges into the edge lists in the vertices
        {
            self.vertex_mut(first)
                .edges
                .insert(half_edge_idx, half_edge_id);

            self.vertex_mut(second)
                .edges
                .insert(half_twin_idx, half_twin_id);

            self.vertex_mut(second)
                .incoming_edges
                .insert(incoming_half_edge_idx, half_edge_id);

            self.vertex_mut(first)
                .incoming_edges
                .insert(incoming_twin_idx, half_twin_id);
        }

        // Fix the `next` of the new edges and of their previous edges
        {
            let first_vertex = self.vertex(first);
            let second_vertex = self.vertex(second);
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

            println!("\nAdding edge:\nid:{half_edge_id:?}\ntwin_id:{half_twin_id:?}\nnext:{next_id:?}\ntwin_next:{twin_next_id:?}\nprev:{prev_id:?}\ntwin_prev:{twin_prev_id:?}");

            // Set the `next` of the previous edges
            self.half_edge_mut(prev_id).next = half_edge_id;
            self.half_edge_mut(twin_prev_id).next = half_twin_id;

            // Set the `prev` of the next edges
            self.half_edge_mut(next_id).prev = half_edge_id;
            self.half_edge_mut(twin_next_id).prev = half_twin_id;

            // Set the `next` of the new edges
            self.half_edge_mut(half_edge_id).next = next_id;
            self.half_edge_mut(half_twin_id).next = twin_next_id;

            // Set the `prev` of the new edges
            self.half_edge_mut(half_edge_id).prev = prev_id;
            self.half_edge_mut(half_twin_id).prev = twin_prev_id;
        }
        // TODO: set the face correctly

        full_edge_id
    }

    // fn add_half_edge(&mut self, origin: VertexId, target: VertexId) -> HalfEdgeId {
    //     let edge_id = HalfEdgeId(self.half_edges.len());

    //     let mut edge = HalfEdge::new(edge_id, origin, target);

    //     self.half_edges.push(edge);

    //     // Find the correct position of the edge

    //     let origin_vertex = self.vertex(origin);
    //     let target_vertex = self.vertex(target);

    //     let edge_idx = match origin_vertex
    //         .edges
    //         .iter()
    //         .map(|&other| self.vertex(self.half_edge(other).target).pos - origin_vertex.pos)
    //         .collect::<Vec<Vec2>>()
    //         .binary_search_by(|other| {
    //             other
    //                 .angle()
    //                 .total_cmp(&(target_vertex.pos - origin_vertex.pos).angle())
    //         }) {
    //         Ok(idx) => idx,
    //         Err(idx) => idx,
    //     };

    //     let twin_idx = match target_vertex
    //         .edges
    //         .iter()
    //         .map(|&other| self.vertex(self.half_edge(other).target).pos - target_vertex.pos)
    //         .collect::<Vec<Vec2>>()
    //         .binary_search_by(|other| {
    //             other
    //                 .angle()
    //                 .total_cmp(&(origin_vertex.pos - target_vertex.pos).angle())
    //         }) {
    //         Ok(idx) => idx,
    //         Err(idx) => idx,
    //     };

    //     let next_idx = (twin_idx + 1) % target_vertex.edges.len(); // TODO: ERROR what to do when there is NO next?

    //     // Insert the half-edges into the outgoing edge lists in the vertices
    //     self.vertex_mut(origin).edges.insert(edge_idx, edge_id);

    //     // Fix the `next` of the new edge and of the previous edge
    //     {
    //         // Find the `next` of the current edges
    //         let target_edges = &self.vertex(target).edges;

    //         // Safety: an item was inserted into the vec previously, and the idx is guaranteed to be in bounds.
    //         let next_id = *target_edges.get(next_idx).unwrap();

    //         let prev_id = self.half_edge(next_id).prev;

    //         println!("\nAdding edge:\nid:{edge_id:?}\nnext:{next_id:?}\nprev:{prev_id:?}");

    //         // Fix the previous and next edges
    //         self.half_edge_mut(prev_id).next = edge_id;
    //         self.half_edge_mut(next_id).prev = edge_id;

    //         // Set the `next` and `prev` of the new edge
    //         self.half_edge_mut(edge_id).next = next_id;
    //         self.half_edge_mut(edge_id).prev = prev_id;
    //     }
    //     // TODO: set the `next` of the half edges correctly
    //     // TODO: set the face correctly

    //     edge_id
    // }

    pub fn iter_vertices(&self) -> Iter<Vertex<VertexData>> {
        self.vertices.iter()
    }

    pub fn iter_mut_vertices(&mut self) -> IterMut<Vertex<VertexData>> {
        self.vertices.iter_mut()
    }

    pub fn iter_edges(&self) -> Iter<Edge> {
        self.edges.iter()
    }

    pub fn vertex(&self, vertex_id: VertexId) -> &Vertex<VertexData> {
        self.vertices.get(vertex_id.0).unwrap()
    }

    pub fn vertex_mut(&mut self, vertex_id: VertexId) -> &mut Vertex<VertexData> {
        self.vertices.get_mut(vertex_id.0).unwrap()
    }

    pub fn half_edge(&self, edge_id: HalfEdgeId) -> &HalfEdge {
        self.half_edges.get(edge_id.0).unwrap()
    }

    pub fn half_edge_mut(&mut self, edge_id: HalfEdgeId) -> &mut HalfEdge {
        self.half_edges.get_mut(edge_id.0).unwrap()
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
