// Edgeself contains what mode can use the edge in each direction
#[derive(Clone)]
pub struct EdgeProperties {
    pub has_railway_tag: bool,
    pub railway_type: String,
    pub usage: String,
    pub service: String,
}

impl Default for EdgeProperties {
    fn default() -> EdgeProperties {
        EdgeProperties {
            has_railway_tag: false,
            railway_type: String::new(),
            usage: String::new(),
            service: String::new(),
        }
    }
}

impl EdgeProperties {
    pub fn update(&mut self, key_string: String, val_string: String) {
        let key = key_string.as_str();
        let val = val_string.as_str();
        self.update_with_str(key, val);
    }

    pub fn update_with_str(&mut self, key: &str, val: &str) {
        match key {
            "railway" => {
                let allowed_railway_types = vec![
                    "light_rail",
                    "monorail",
                    "narrow_gauge",
                    "rail",
                    "subway",
                    "tram",
                ];

                if allowed_railway_types.contains(&val) {
                    self.has_railway_tag = true;
                    self.railway_type = val.to_string()
                }
            }
            "usage" => self.usage = val.to_string(),
            "service" => self.service = val.to_string(),
            _ => {}
        }
    }
}
