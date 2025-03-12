use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelResponse {
    object: String,
    data: Vec<Data>,
}

impl ModelResponse {
    pub fn object(&self) -> &str {
        &self.object
    }
    pub fn data(&self) -> &[Data] {
        &self.data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    id: String,
    object: String,
    owned_by: String,
}

impl Data {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn object(&self) -> &str {
        &self.object
    }
    pub fn onwed_by(&self) -> &str {
        &self.owned_by
    }
}