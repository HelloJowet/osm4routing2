use super::super::categorize::edge_properties::EdgeProperties;
use super::super::models::edge::Edge;
use crate::ProfileType;

pub fn create_edges_csv(edges: Vec<Edge>, profile_type: ProfileType) {
    let edges_path = std::path::Path::new("edges.csv");
    let mut edges_csv = csv::Writer::from_path(edges_path).unwrap();

    let edges_csv_columns = match profile_type {
        ProfileType::Railway => vec!["railway_type", "usage", "service", "geometry"],
        ProfileType::Road => vec![
            "id",
            "osm_id",
            "source",
            "target",
            "foot",
            "car_forward",
            "car_backward",
            "bike_forward",
            "bike_backward",
            "train",
            "wkt",
        ],
    };

    edges_csv
        .serialize(edges_csv_columns)
        .expect("CSV: unable to write edge header");

    for edge in &edges {
        match &edge.properties {
            EdgeProperties::RailwayEdgeProperties(edge_properties) => {
                edges_csv
                    .serialize((
                        edge_properties.railway_type.clone(),
                        edge_properties.usage.clone(),
                        edge_properties.service.clone(),
                        edge.as_wkt(),
                    ))
                    .expect("CSV: unable to write edge");
            }
            EdgeProperties::RoadEdgeProperties(edge_properties) => {
                edges_csv
                    .serialize((
                        &edge.id,
                        edge.osm_id.0,
                        edge.source.0,
                        edge.target.0,
                        edge_properties.foot,
                        edge_properties.car_forward,
                        edge_properties.car_backward,
                        edge_properties.bike_forward,
                        edge_properties.bike_backward,
                        edge_properties.train,
                        edge.as_wkt(),
                    ))
                    .expect("CSV: unable to write edge");
            }
        }
    }
}
