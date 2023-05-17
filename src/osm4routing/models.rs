use super::categorize::EdgeProperties;
use osmpbfreader::objects::{NodeId, WayId};

// Coord are coordinates in decimal degress WGS84
#[derive(Copy, Clone, Default)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

// Node is the OpenStreetMap node
#[derive(Copy, Clone)]
pub struct Node {
    pub id: NodeId,
    pub coord: Coord,
    pub uses: i16,
}

impl Default for Node {
    fn default() -> Node {
        Node {
            id: NodeId(0),
            coord: Default::default(),
            uses: Default::default(),
        }
    }
}

// Edge is a topological representation with only two extremities and no geometry
pub struct Edge {
    pub id: String,
    pub osm_id: WayId,
    pub source: NodeId,
    pub target: NodeId,
    pub geometry: Vec<Coord>,
    pub properties: EdgeProperties,
}

impl Edge {
    // Geometry in the well known format
    pub fn as_wkt(&self) -> String {
        let coords: Vec<String> = self
            .geometry
            .iter()
            .map(|coord| format!("{:.7} {:.7}", coord.lon, coord.lat))
            .collect();

        format!("LINESTRING({})", coords.as_slice().join(", "))
    }

    // Length in meters of the edge
    pub fn length(&self) -> f64 {
        self.geometry
            .windows(2)
            .map(|coords| distance(coords[0], coords[1]))
            .sum()
    }
}

fn distance(start: Coord, end: Coord) -> f64 {
    let r: f64 = 6_378_100.0;

    let d_lon: f64 = (end.lon - start.lon).to_radians();
    let d_lat: f64 = (end.lat - start.lat).to_radians();
    let lat1: f64 = (start.lat).to_radians();
    let lat2: f64 = (end.lat).to_radians();

    let a: f64 = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
        + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
    let c: f64 = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));

    r * c
}
