use super::models::*;

pub fn csv(nodes: Vec<Node>, edges: Vec<Edge>) {
    let edges_path = std::path::Path::new("edges.csv");
    let mut edges_csv = csv::Writer::from_path(edges_path).unwrap();
    edges_csv
        .serialize(vec![
            "railway_type",
            "usage",
            "service",
            "geometry",
            "approximate_centerpoint_geohash",
        ])
        .expect("CSV: unable to write edge header");
    for edge in edges {
        edges_csv
            .serialize((
                edge.properties.railway_type.clone(),
                edge.properties.usage.clone(),
                edge.properties.service.clone(),
                edge.as_wkt(),
                edge.get_approximate_centerpoint_geohash(),
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
