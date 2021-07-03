use derive_more::Deref;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(test)]
mod tests;

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct Title {
    title: Vec<RichText>,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    Default,
    Gray,
    Brown,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Red,
    #[serde(other)]
    Unknown,
}

pub type RichTexts = Vec<RichText>;

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Default, Serialize, Deserialize)]
pub struct RichTextCommon {
    pub plain_text: Option<String>,
    pub annotations: Option<Annotation>,
    pub href: Option<String>,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichText {
    Text {
        text: TextInner,
        #[serde(flatten)]
        common: RichTextCommon,
    },
    Mention {
        mention: MentionKind,
        #[serde(flatten)]
        common: RichTextCommon,
    },
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MentionKind {
    User { user: crate::User },
    Page { page: Page },
    Database { database: Database },
    Date { date: crate::property_values::Date },
}

// #[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
// #[cfg_attr(test, derive(PartialEq))]
// #[derive(Serialize, Deserialize, Deref)]
// pub struct RichText {
//     #[deref]
//     pub text: TextInner,
//     pub plain_text: Option<String>,
//     pub annotations: Option<Annotation>,
//     pub href: Option<String>,
// }

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize, Deref)]
pub struct Page {
    #[deref]
    pub id: Uuid,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize, Deref)]
pub struct Database {
    #[deref]
    pub id: Uuid,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize, Deref)]
pub struct TextInner {
    #[deref]
    pub content: String,
    pub link: Option<Link>,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize, Deref)]
pub struct Link {
    #[deref]
    url: String,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Default, Serialize, Deserialize)]
pub struct Annotation {
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub strikethrough: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub code: bool,
    pub color: Option<Color>,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize, Deref)]
pub struct Select {
    #[deref]
    pub select: SelectInner,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize, Deref)]
pub struct SelectInner {
    #[deref]
    pub name: String,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize, Deref)]
pub struct Number {
    #[deref]
    pub number: f64,
}
