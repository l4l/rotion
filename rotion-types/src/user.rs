use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(test)]
mod tests;

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    #[serde(flatten)]
    pub kind: UserKind,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UserKind {
    Person {
        person: Person,
    },
    Bot {},
    #[serde(other)]
    Unknown,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct Person {
    pub email: String,
}
