use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::PropertyValues;

#[cfg(test)]
mod tests;

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct Page {
    pub id: Uuid,
    pub created_time: DateTime<Utc>,
    pub last_edited_time: DateTime<Utc>,
    pub archived: bool,
    pub properties: PropertyValues,
    pub parent: Parent,
    pub url: String,
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
    DatabaseId {
        database_id: Uuid,
    },
    #[serde(other)]
    Unknown,
}
