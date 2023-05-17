use super::categorize::*;
use super::models::*;
use osmpbfreader::objects::{NodeId, Tags, WayId};
use std::collections::{HashMap, HashSet};

// Way as represented in OpenStreetMap
struct Way {
    id: WayId,
    nodes: Vec<NodeId>,
    properties: EdgeProperties,
    tags: Tags,
}

pub struct Reader {
    nodes: HashMap<NodeId, Node>,
    ways: Vec<Way>,
    nodes_to_keep: HashSet<NodeId>,
    forbidden: HashMap<String, HashSet<String>>,
}

impl Default for Reader {
    fn default() -> Self {
        Self::new()
    }
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            nodes: HashMap::new(),
            ways: Vec::new(),
            nodes_to_keep: HashSet::new(),
            forbidden: HashMap::new(),
        }
    }

    pub fn reject(mut self, key: &str, value: &str) -> Self {
        self.forbidden
            .entry(key.to_string())
            .or_default()
            .insert(value.to_string());
        self
    }

    fn count_nodes_uses(&mut self) {
        for way in &self.ways {
            for (i, node_id) in way.nodes.iter().enumerate() {
                if let Some(node) = self.nodes.get_mut(node_id) {
                    // Count double extremities nodes
                    if i == 0 || i == way.nodes.len() - 1 {
                        node.uses += 2;
                    } else {
                        node.uses += 1;
                    }
                } else {
                    panic!("Missing node, id: {:?}", node_id)
                }
            }
        }
    }

    fn split_way(&self, way: &Way) -> Vec<Edge> {
        let mut result = Vec::new();

        let mut source = NodeId(0);
        let mut geometry = Vec::new();
        for (i, &node_id) in way.nodes.iter().enumerate() {
            let node = self.nodes[&node_id];
            if i == 0 {
                source = node_id;
                geometry.push(node.coord);
            } else {
                geometry.push(node.coord);

                if node.uses > 1 {
                    let tag_usage = way.tags.get("usage");
                    let mut tag_usage_value = String::new();
                    if !!!tag_usage.is_none() {
                        tag_usage_value = tag_usage.unwrap().to_string()
                    }

                    result.push(Edge {
                        id: format!("{}-{}", way.id.0, result.len()),
                        osm_id: way.id,
                        source,
                        target: node_id,
                        geometry,
                        properties: way.properties,
                        tags: tag_usage_value,
                    });

                    source = node_id;
                    geometry = vec![node.coord];
                }
            }
        }
        result
    }

    fn read_ways(&mut self, file: std::fs::File) {
        let mut pbf = osmpbfreader::OsmPbfReader::new(file);
        for obj in pbf.iter().flatten() {
            if let osmpbfreader::OsmObj::Way(way) = obj {
                let mut skip = false;
                let mut properties: EdgeProperties = EdgeProperties::default();
                for (key, val) in way.tags.iter() {
                    properties.update(key.to_string(), val.to_string());
                    if self
                        .forbidden
                        .get(key.as_str())
                        .map(|vals| vals.contains(val.as_str()) || vals.contains("*"))
                        == Some(true)
                    {
                        skip = true;
                    }
                }
                properties.normalize();
                if properties.accessible() && !skip {
                    for node in &way.nodes {
                        self.nodes_to_keep.insert(*node);
                    }
                    self.ways.push(Way {
                        id: way.id,
                        nodes: way.nodes,
                        properties,
                        tags: way.tags,
                    });
                }
            }
        }
    }

    fn read_nodes(&mut self, file: std::fs::File) {
        let mut pbf = osmpbfreader::OsmPbfReader::new(file);
        self.nodes.reserve(self.nodes_to_keep.len());
        for obj in pbf.iter().flatten() {
            if let osmpbfreader::OsmObj::Node(node) = obj {
                if self.nodes_to_keep.contains(&node.id) {
                    self.nodes_to_keep.remove(&node.id);
                    self.nodes.insert(
                        node.id,
                        Node {
                            id: node.id,
                            coord: Coord {
                                lon: node.lon(),
                                lat: node.lat(),
                            },
                            uses: 0,
                        },
                    );
                }
            }
        }
    }

    fn nodes(self) -> Vec<Node> {
        self.nodes
            .into_values()
            .filter(|node| node.uses > 1)
            .collect()
    }

    fn edges(&self) -> Vec<Edge> {
        self.ways
            .iter()
            .flat_map(|way| self.split_way(way))
            .collect()
    }

    pub fn read(mut self, filename: &str) -> Result<(Vec<Node>, Vec<Edge>), String> {
        let path = std::path::Path::new(filename);
        let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
        self.read_ways(file);
        let file_nodes = std::fs::File::open(path).map_err(|e| e.to_string())?;
        self.read_nodes(file_nodes);
        self.count_nodes_uses();
        let edges = self.edges();
        Ok((self.nodes(), edges))
    }
}

// Read all the nodes and ways of the osm.pbf file
pub fn read(filename: &str) -> Result<(Vec<Node>, Vec<Edge>), String> {
    Reader::new().read(filename)
}
