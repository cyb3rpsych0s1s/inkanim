use serde::{
    Deserialize, Serialize,
    ser::{SerializeSeq, SerializeStruct},
};

use crate::{HandleId, Name};

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
#[serde(tag = "$type", rename_all = "camelCase")]
pub struct inkPropertyBinding {
    pub property_name: Name,
    pub style_path: Name,
}

impl Serialize for inkPropertyBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("inkPropertyBinding", 2)?;
        s.serialize_field("propertyName", &self.property_name)?;
        s.serialize_field("stylePath", &self.style_path)?;
        s.end()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
#[serde(tag = "$type")]
pub struct inkPropertyManager {
    pub bindings: Vec<inkPropertyBinding>,
}

impl Serialize for inkPropertyManager {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.bindings.len()))?;
        for elem in self.bindings.iter() {
            seq.serialize_element(elem)?;
        }
        seq.end()
    }
}

#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyManager {
    pub handle_id: HandleId,
    pub data: inkPropertyManager,
}

impl Serialize for PropertyManager {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.data.serialize(serializer)
    }
}
