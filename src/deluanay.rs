use std::ops::{Deref, DerefMut};

use crate::dcel::{edge::EdgeId, graph::GeometricGraph};

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
    fn triangulate(self) -> Triangulated<Self> {
        Triangulated(self)
        // todo!();
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
        let flip_source_node = self.view_edge(edge).half_edge().next().target();
        let flip_target_node = self.view_edge(edge).twin_edge().next().target();

        self.remove_edge(edge);
        self.add_edge(flip_source_node, flip_target_node)
    }

    fn edge_deluanay_condition(&self, edge: EdgeId) -> bool {
        todo!()
    }
}
