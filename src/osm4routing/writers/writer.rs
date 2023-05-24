use super::super::models::{edge::Edge, node::Node};
use super::{edges, geohashes, nodes};
use crate::ProfileType;

pub struct Writer {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    profile_type: ProfileType,
}

impl Writer {
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>, profile_type: ProfileType) -> Writer {
        Writer {
            nodes: nodes,
            edges: edges,
            profile_type: profile_type,
        }
    }

    pub fn to_csv(&self) {
        let geohash_precision: u8 = 24;

        let mut geohashes: Vec<String> = Vec::new();
        for edge in self.edges.iter() {
            let mut edge_geohashes = edge.get_geohashes(geohash_precision);
            geohashes.append(&mut edge_geohashes);
        }

        nodes::create_nodes_csv(self.nodes.clone());
        edges::create_edges_csv(self.edges.clone(), self.profile_type);
        geohashes::create_geohashes_csv(geohashes, geohash_precision);
    }
}

pub fn write(nodes: Vec<Node>, edges: Vec<Edge>, profile_type: ProfileType) {
    Writer::new(nodes, edges, profile_type).to_csv();
}
