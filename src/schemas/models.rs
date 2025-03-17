use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Models {
    pub object: String,
    pub data: Vec<Model>,
}

#[derive(Deserialize, Serialize)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub owned_by: String,
}
