use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use super::InkWrapper;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InkAnimSequenceTargetInfo {
    pub path: Vec<usize>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct inkanimScaleInterpolator_Data {}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct inkanimTranslationInterpolator_Data {}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct inkanimTransparencyInterpolator_Data {}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct inkanimSizeInterpolator_Data {}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct inkanimColorInterpolator_Data {}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct inkanimTextValueProgressInterpolator_Data {}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum InkAnimInterpolator {
    inkanimScaleInterpolator(inkanimScaleInterpolator_Data),
    inkanimTranslationInterpolator(inkanimTranslationInterpolator_Data),
    inkanimTransparencyInterpolator(inkanimTransparencyInterpolator_Data),
    inkanimSizeInterpolator(inkanimSizeInterpolator_Data),
    inkanimColorInterpolator(inkanimColorInterpolator_Data),
    inkanimTextValueProgressInterpolator(inkanimTextValueProgressInterpolator_Data),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InkAnimDefinition {
    pub interpolators: Vec<InkWrapper<InkAnimInterpolator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InkAnimSequence {
    pub definitions: Vec<InkWrapper<InkAnimDefinition>>,
    pub name: String,
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InkAnimAnimationLibraryResource {
    pub cooking_platform: String,
    pub sequences: Vec<InkWrapper<InkAnimSequence>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlankInkAnimSequenceTargetInfo {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub handle_ref_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Target {
    WithHandleId(InkWrapper<InkAnimSequenceTargetInfo>),
    WithoutHandleId(BlankInkAnimSequenceTargetInfo),
}

impl InkAnimSequence {
    pub fn get_path_indexes_matching(&self, searched: &[usize]) -> Vec<PathSummary> {
        let count = searched.len();
        let last = count - 1;
        let mut out = vec![];
        for (target_index, target) in self.targets.iter().enumerate() {
            match target {
                Target::WithHandleId(ref handle) => {
                    let ref path = handle.data.path;
                    'inner: for (i, path_index) in searched.iter().enumerate() {
                        if path_index != &path[i] {
                            break 'inner;
                        }
                        if i == last {
                            let summary = PathSummary {
                                HandleId: handle.handle_id,
                                Index: target_index,
                                Path: path.clone(),
                            };
                            out.push(summary);
                            break 'inner;
                        }
                    }
                }
                _ => continue,
            }
        }
        return out;
    }
}

#[allow(dead_code, non_snake_case)]
#[derive(Debug)]
pub struct PathSummary {
    HandleId: u32,
    Index: usize,
    Path: Vec<usize>,
}
