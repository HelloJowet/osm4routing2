use super::super::models::edge::Edge;

pub fn create_edges_csv(edges: Vec<Edge>) {
    let edges_path = std::path::Path::new("edges.csv");
    let mut edges_csv = csv::Writer::from_path(edges_path).unwrap();
    edges_csv
        .serialize(vec!["railway_type", "usage", "service", "geometry"])
        .expect("CSV: unable to write edge header");

    for edge in edges {
        edges_csv
            .serialize((
                edge.properties.railway_type.clone(),
                edge.properties.usage.clone(),
                edge.properties.service.clone(),
                edge.as_wkt(),
            ))
            .expect("CSV: unable to write edge");
    }
}
