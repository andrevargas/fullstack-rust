use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Tweet {
    pub content: String,
}