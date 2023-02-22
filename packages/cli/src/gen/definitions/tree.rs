use std::ops::{Deref, DerefMut};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use super::{
    builder::DefinitionsItemBuilder, item::DefinitionsItem, kind::DefinitionsItemKind,
    parser::parse_type_definitions_into_doc_items,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefinitionsTree(DefinitionsItem);

#[allow(dead_code)]
impl DefinitionsTree {
    pub fn from_type_definitions<S: AsRef<str>>(type_definitions_contents: S) -> Result<Self> {
        let top_level_items = parse_type_definitions_into_doc_items(type_definitions_contents)
            .context("Failed to visit type definitions AST")?;
        let root = DefinitionsItemBuilder::new()
            .with_kind(DefinitionsItemKind::Root)
            .with_name("<<<ROOT>>>")
            .with_children(&top_level_items)
            .build()?;
        Ok(Self(root))
    }

    #[allow(clippy::unused_self)]
    pub fn is_root(&self) -> bool {
        true
    }

    pub fn into_inner(self) -> DefinitionsItem {
        self.0
    }
}

impl Deref for DefinitionsTree {
    type Target = DefinitionsItem;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DefinitionsTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}