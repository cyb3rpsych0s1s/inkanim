use serde::{Deserialize, Deserializer, Serialize};
use serde_aux::prelude::*;

pub mod anim;
pub mod widget;

pub use anim::*;
pub use widget::*;

pub fn deserialize_handle_id_from_string<'de, D>(deserializer: D) -> Result<HandleId, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(HandleId(deserialize_number_from_string(deserializer)?))
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HandleId(u32);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InkWrapper<T> {
    #[serde(deserialize_with = "deserialize_handle_id_from_string")]
    pub handle_id: HandleId,
    pub data: T,
}

impl<T> std::fmt::Display for InkWrapper<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) {}", self.handle_id, self.data)
    }
}

impl std::fmt::Display for HandleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "🔑 {}", self.0)
    }
}
