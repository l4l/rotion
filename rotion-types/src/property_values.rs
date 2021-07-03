use std::collections::HashMap;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user::User;
use crate::values::{Color, RichTexts};
use crate::PropertyName;

pub type PropertyValues = HashMap<PropertyName, PropertyValue>;

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PropertyValue {
    Title { title: RichTexts },
    RichText { rich_text: RichTexts },
    Number { number: f64 },
    Select { multi_select: SelectVariant },
    MultiSelect { multi_select: SelectVariant },
    Date { date: Date },
    Formula { formula: Formula },
    Relation { relation: Vec<RelationRef> },
    Rollup { rollup: Rollup },
    People { people: Vec<User> },
    Files { file: Vec<File> },
    Checkbox { checkbox: bool },
    Url { url: String },
    Email { email: String },
    PhoneNumber { phone_number: String },
    CreatedTime { created_time: DateTime<Utc> },
    CreatedBy { created_by: User },
    LastEditedTime { last_edited_time: DateTime<Utc> },
    LastEditedBy { created_by: User },
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct SelectVariant {
    pub id: Uuid,
    pub name: String,
    pub color: Color,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct Date {
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Formula {
    String { string: String },
    Number { number: f64 },
    Boolean { boolean: bool },
    Date { date: DateTime<Utc> },
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct RelationRef {
    #[serde(rename = "id")]
    pub page_id: Uuid,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Rollup {
    Number { number: f64 },
    Date { date: DateTime<Utc> },
    Array { array: Vec<PropertyValue> },
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct File {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property() {
        let s = r#"{
  "id": "title",
  "type": "title",
  "title": [
    {
      "type": "text",
      "text": {
        "content": "Task management ",
        "link": null
      },
      "annotations": {
        "bold": false,
        "italic": false,
        "strikethrough": false,
        "underline": false,
        "code": false,
        "color": "default"
      },
      "plain_text": "Task management ",
      "href": null
    },
    {
      "type": "mention",
      "mention": {
        "type": "user",
        "user": {
          "object": "user",
          "id": "e469e4ae-6445-487d-9904-e6bc2ec8a9de",
          "name": "Eugene Enegue",
          "avatar_url": "https://lh6.googleusercontent.com/-K6VrPgWalmw/AAAAAAAAAAI/AAAAAAAAAAA/AMZuucmaDL0qknFz71Hou6FODGkzALtfYw/photo.jpg",
          "type": "person",
          "person": {
            "email": "justkitsu@gmail.com"
          }
        }
      },
      "annotations": {
        "bold": false,
        "italic": false,
        "strikethrough": false,
        "underline": false,
        "code": false,
        "color": "default"
      },
      "plain_text": "@Eugene Enegue",
      "href": null
    },
    {
      "type": "text",
      "text": {
        "content": " ",
        "link": null
      },
      "annotations": {
        "bold": false,
        "italic": false,
        "strikethrough": false,
        "underline": false,
        "code": false,
        "color": "default"
      },
      "plain_text": " ",
      "href": null
    },
    {
      "type": "text",
      "text": {
        "content": "qw",
        "link": null
      },
      "annotations": {
        "bold": true,
        "italic": false,
        "strikethrough": false,
        "underline": false,
        "code": false,
        "color": "default"
      },
      "plain_text": "qw",
      "href": null
    }
  ]
}
"#;

        serde_json::from_str::<PropertyValue>(s).unwrap();
    }
}
