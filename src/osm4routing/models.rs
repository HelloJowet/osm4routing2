use super::categorize::EdgeProperties;
use geohashrust::{BinaryHash, GeoLocation};
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

    pub fn get_geohashes(&self, precision: u8) -> Vec<String> {
        let mut geohashes: Vec<String> = Vec::new();

        for coord in self.geometry.clone().iter_mut() {
            let geo_location = GeoLocation {
                latitude: coord.lat,
                longitude: coord.lon,
            };
            let binary_hash = BinaryHash::encode(&geo_location, precision);

            geohashes.push(binary_hash.to_string());
        }

        // remove duplicated geohashes
        geohashes.sort_unstable();
        geohashes.dedup();

        return geohashes;
    }
}
