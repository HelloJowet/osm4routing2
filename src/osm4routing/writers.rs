use super::models::*;

pub fn csv(nodes: Vec<Node>, edges: Vec<Edge>) {
    let edges_path = std::path::Path::new("edges.csv");
    let mut edges_csv = csv::Writer::from_path(edges_path).unwrap();
    edges_csv
        .serialize(vec![
            "id", "osm_id", "source", "target", "length", "railway", "tags", "wkt",
        ])
        .expect("CSV: unable to write edge header");
    for edge in edges {
        edges_csv
            .serialize((
                &edge.id,
                edge.osm_id.0,
                edge.source.0,
                edge.target.0,
                edge.length(),
                edge.properties.railway,
                edge.tags.clone(),
                edge.as_wkt(),
            ))
            .expect("CSV: unable to write edge");
    }

    let nodes_path = std::path::Path::new("nodes.csv");
    let mut nodes_csv = csv::Writer::from_path(nodes_path).unwrap();
    nodes_csv
        .serialize(vec!["id", "lon", "lat"])
        .expect("CSV: unable to write node header");
    for node in nodes {
        nodes_csv
            .serialize((node.id.0, node.coord.lon, node.coord.lat))
            .expect("CSV: unable to write node");
    }
}

// pub fn pg(nodes: Vec<Node>, edges: Vec<Edge>) {}
