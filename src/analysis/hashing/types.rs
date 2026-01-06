use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityResult {
    pub is_similar: bool,
    pub similarity_score: f64,
}


