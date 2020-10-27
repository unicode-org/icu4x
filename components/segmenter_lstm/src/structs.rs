use ndarray::Array2;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct PlaceholderData {
    pub a: f32,
    pub b: Array2<f32>,
}
