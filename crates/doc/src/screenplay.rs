//! Screenplay roles on ordinary document paragraphs.

use serde::{Deserialize, Serialize};

/// A paragraph's role in a script, independent of its formatting.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScriptParagraphKind {
    General,
    SceneHeading,
    Action,
    Character,
    Parenthetical,
    Dialogue,
    Transition,
    Shot,
    /// A role name outside the built-in vocabulary.
    Custom(String),
}

/// Screenplay data attached to an ordinary paragraph.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptParagraph {
    pub kind: ScriptParagraphKind,
    /// An identifier from the source, never used as an internal editing key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// A production label, independent of document order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scene_number: Option<String>,
}

impl ScriptParagraph {
    pub fn new(kind: ScriptParagraphKind) -> Self {
        Self { kind, source_id: None, scene_number: None }
    }
}
