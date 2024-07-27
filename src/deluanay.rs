use std::ops::{Deref, DerefMut};

use nannou::{
    glam::Vec2,
    math::{num_traits::Signed, Vec2Angle},
};

use crate::dcel::{edge::EdgeId, graph::GeometricGraph, vertex::VertexId};

pub struct Triangulated<T>(T);

impl<T> Deref for Triangulated<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Triangulated<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait Triangulable
where
    Self: Sized,
{
    fn triangulate(self) -> Triangulated<Self>;
}

impl<V> Triangulable for GeometricGraph<V> {
    fn triangulate(mut self) -> Triangulated<Self> {
        self.remove_all_edges();

        if self.num_vertices() <= 1 {
            return Triangulated(self); // Nothing to triangulate here, at most a single vertex...
        }

        // Generate an initial triangulation for the body by choosing a root and connecting everything to it
        let root = triangulate_body(&mut self);

        // Need to fix the hull's triangulation as well!
        // Go through the hull, starting at the rightmost edge (which is guaranteed to be the first edge on the root by construction) and going around on it
        triangulate_hull(&mut self, root);

        Triangulated(self)
    }
}

fn triangulate_body<V>(graph: &mut GeometricGraph<V>) -> VertexId {
    // Temp typedef for readability
    struct VertexPoint {
        pos: Vec2,
        id: VertexId,
    }

    let mut points: Vec<VertexPoint> = graph
        .iter_vertices()
        .map(|v| VertexPoint {
            pos: v.pos,
            id: v.id(),
        })
        .collect();

    if points.len() <= 1 {}

    points.sort_unstable_by(|v1, v2| v1.pos.x.partial_cmp(&v2.pos.x).unwrap());
    let root = points.pop().unwrap();

    points.sort_unstable_by(|v1, v2| {
        (root.pos - v1.pos)
            .angle()
            .partial_cmp(&(root.pos - v2.pos).angle())
            .unwrap()
    });

    for cur_points in points.windows(2) {
        graph.add_edge(root.id, cur_points[0].id);
        graph.add_edge(cur_points[0].id, cur_points[1].id);
    }

    // Last triangle is missed in the loop, need to add it manually.
    let last_point = points.last().unwrap();
    graph.add_edge(root.id, last_point.id);

    root.id
}

fn triangulate_hull<V>(graph: &mut GeometricGraph<V>, root: VertexId) {
    // Subproblems are excludive - each subproblem is of the form [start_edge.origin, end_vertex) excluding the end vertex
    let mut subproblems = vec![(
        graph.view_vertex(root).first_edge().unwrap(),
        graph.view_vertex(root),
    )];

    let mut edges_to_add = Vec::new();

    while let Some((start_edge, end_vertex)) = subproblems.pop() {
        let start_vertex = start_edge.origin();
        let mut cur_edge = start_edge.next(); // We can skip the first edge because its known to be rightmost

        let mut edges_with_rightmost_target = vec![start_edge];
        while cur_edge.target().id() != end_vertex.id() {
            let last_rightmost = *edges_with_rightmost_target.last().unwrap();

            if is_right_of_line(
                start_vertex.pos(),
                last_rightmost.target().pos(),
                cur_edge.target().pos(),
            ) {
                edges_with_rightmost_target.push(cur_edge);
                edges_to_add.push((start_vertex.id(), cur_edge.target().id()));

                // Because the subproblem is excluding the end, i.e [start, end) then need to create [start, end + 1) instead
                subproblems.push((last_rightmost.next(), cur_edge.next().target()));
            }

            cur_edge = cur_edge.next();
        }

        let last = *edges_with_rightmost_target.last().unwrap();
        if last.next().target().id() != end_vertex.id() {
            subproblems.push((last.next(), end_vertex));
        }
    }

    for (origin, target) in edges_to_add {
        graph.add_edge(origin, target);
    }
}

pub trait IntoDeluanay {
    fn into_deluanay(self) -> Self;
    fn flip_edge(&mut self, edge: EdgeId) -> EdgeId;
    fn edge_deluanay_condition(&self, edge: EdgeId) -> bool;
}

impl<V> IntoDeluanay for Triangulated<GeometricGraph<V>> {
    fn into_deluanay(mut self) -> Self {
        let mut bad_edges = self
            .iter_edges()
            .map(|edge| edge.id())
            .filter(|&edge| self.edge_deluanay_condition(edge))
            .collect::<Vec<EdgeId>>();

        while !bad_edges.is_empty() {
            let mut edges_to_check = Vec::new();
            for &edge in bad_edges.iter() {
                if self.edge_deluanay_condition(edge) {
                    let new_edge_id = self.flip_edge(edge);
                    let new_edge = self.view_edge(new_edge_id);

                    edges_to_check.push(new_edge.next().id());
                    edges_to_check.push(new_edge.prev().id());
                    edges_to_check.push(new_edge.twin_next().id());
                    edges_to_check.push(new_edge.twin_prev().id());
                }
            }
            bad_edges = edges_to_check
        }

        self
    }

    fn flip_edge(&mut self, edge: EdgeId) -> EdgeId {
        let flip_source_node = self.view_edge(edge).half_edge().next().target().id();
        let flip_target_node = self.view_edge(edge).twin_edge().next().target().id();

        self.remove_edge(edge);
        self.add_edge(flip_source_node, flip_target_node)
    }

    // TODO: this method is buggy. Figure & fix
    fn edge_deluanay_condition(&self, edge_id: EdgeId) -> bool {
        let edge = self.view_edge(edge_id);

        // TODO: add check that edge's faces are not the outer face. If any face is the outer face - should return true (because flipping is meaningless here)

        let a = edge.origin().pos();
        let b = edge.target().pos();
        let c = edge.next().target().pos();

        let testing_point = edge.twin_next().target().pos();

        in_circle(a, b, c, testing_point)
    }
}

fn in_circle(a: Vec2, b: Vec2, c: Vec2, testing_point: Vec2) -> bool {
    let a_diff = a - testing_point;
    let b_diff = b - testing_point;
    let c_diff = c - testing_point;

    (a_diff.length_squared() * b_diff.perp_dot(c_diff)
        - b_diff.length_squared() * a_diff.perp_dot(c_diff)
        + c_diff.length_squared() * a_diff.perp_dot(b_diff))
    .is_positive()
}

fn is_right_of_line(origin: Vec2, target: Vec2, test_point: Vec2) -> bool {
    let diff_target = target - origin;
    let diff_test = test_point - origin;
    let product = diff_target.perp_dot(diff_test);

    product.is_negative()
}
