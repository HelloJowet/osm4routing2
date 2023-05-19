use super::categorize::EdgeProperties;
use geohash::{encode, Coord as GeohashCoord};
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

    pub fn get_approximate_centerpoint_geohash(&self) -> String {
        let line_geometry_coord_count: usize = self.geometry.len();
        let line_geometry_approximate_centerpoint = self.geometry[line_geometry_coord_count / 2];
        let approximate_centerpoint_geohash_coord_object = GeohashCoord {
            x: line_geometry_approximate_centerpoint.lon,
            y: line_geometry_approximate_centerpoint.lat,
        };

        match encode(approximate_centerpoint_geohash_coord_object, 9usize) {
            Err(error) => panic!("Problem creating geohash: {:?}", error),
            Ok(f) => f,
        }
    }
}
