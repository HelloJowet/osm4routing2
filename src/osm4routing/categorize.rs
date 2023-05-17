use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq)]
pub enum TrainAccessibility {
    Unknown,
    Forbidden,
    Allowed,
}

// Edgeself contains what mode can use the edge in each direction
#[derive(Clone, Copy)]
pub struct EdgeProperties {
    pub railway: TrainAccessibility,
}

impl Default for EdgeProperties {
    fn default() -> EdgeProperties {
        EdgeProperties {
            railway: TrainAccessibility::Unknown,
        }
    }
}

impl EdgeProperties {
    // Normalize fills UNKNOWN fields
    pub fn normalize(&mut self) {}

    // Accessible means that at least one mean of transportation can use it in one direction
    pub fn accessible(self) -> bool {
        self.railway == TrainAccessibility::Allowed
    }

    pub fn update(&mut self, key_string: String, val_string: String) {
        let key = key_string.as_str();
        let val = val_string.as_str();
        self.update_with_str(key, val);
    }

    pub fn update_with_str(&mut self, key: &str, val: &str) {
        match key {
            "railway" => self.railway = TrainAccessibility::Allowed,
            _ => {}
        }
    }
}
