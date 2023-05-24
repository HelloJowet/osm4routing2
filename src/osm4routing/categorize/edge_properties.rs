use super::railway::edge_properties::EdgeProperties as RailwayEdgeProperties;
use super::road::edge_properties::EdgeProperties as RoadEdgeProperties;

#[derive(Clone)]
pub enum EdgeProperties {
    RailwayEdgeProperties(RailwayEdgeProperties),
    RoadEdgeProperties(RoadEdgeProperties),
}
