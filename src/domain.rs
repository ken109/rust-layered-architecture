use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub(crate) id: Option<u32>,
    pub(crate) content: String,
    pub(crate) done: Option<bool>,
}
