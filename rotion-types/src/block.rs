use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::RichTexts;

#[cfg(test)]
mod tests;

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct Block {
    pub id: Uuid,
    pub created_time: DateTime<Utc>,
    pub last_edited_time: DateTime<Utc>,
    // pub has_children: bool,
    #[serde(flatten)]
    pub block_data: BlockData,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlockData {
    Paragraph {
        paragraph: NestedItem,
    },
    #[serde(rename = "heading_1")]
    Heading1 {
        heading_1: Heading,
    },
    #[serde(rename = "heading_2")]
    Heading2 {
        heading_2: Heading,
    },
    #[serde(rename = "heading_3")]
    Heading3 {
        heading_3: Heading,
    },
    BulletedListItem {
        bulleted_list_item: NestedItem,
    },
    NumberedNestedItem {
        numbered_list_item: NestedItem,
    },
    ToDo {
        to_do: ToDo,
    },
    Toggle {
        toggle: NestedItem,
    },
    ChildPage {
        child_page: ChildPage,
    },
    Unsupported {},
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct Heading {
    pub text: RichTexts,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct NestedItem {
    pub text: RichTexts,
    #[serde(default)]
    pub children: Vec<Block>,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct ToDo {
    pub text: RichTexts,
    #[serde(default)]
    pub checked: bool,
    #[serde(default)]
    pub children: Vec<Block>,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct ChildPage {
    pub title: String,
}
