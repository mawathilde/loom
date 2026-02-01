#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Health {
    pub status: String,
}