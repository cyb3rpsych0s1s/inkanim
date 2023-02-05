use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

pub mod anim;
pub mod widget;

pub use anim::*;
pub use widget::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InkWrapper<T> {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub handle_id: u32,
    pub data: T,
}

impl<T> std::fmt::Display for InkWrapper<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (HandleId: {})", self.data, self.handle_id)
    }
}
