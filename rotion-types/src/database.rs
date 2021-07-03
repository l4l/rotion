use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Properties, RichTexts};

#[cfg(test)]
mod tests;

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct Database {
    pub id: Uuid,
    pub created_time: DateTime<Utc>,
    pub last_edited_time: DateTime<Utc>,
    pub title: RichTexts,
    pub properties: Properties,
    pub parent: Parent,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Parent {
    PageId {
        page_id: Uuid,
    },
    Workspace {
        workspace: bool,
    },
    #[serde(other)]
    Unknown,
}
