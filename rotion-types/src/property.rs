use std::collections::HashMap;

use derive_more::Deref;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::values::Color;
use crate::{PropertyId, PropertyName};

pub type Properties = HashMap<PropertyName, Property>;

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[rustfmt::skip]
pub enum Property {
    Title { id: PropertyId },
    #[serde(rename = "rich_text")]
    Text { id: PropertyId },
    Number { id: PropertyId, number: Number },
    Select { id: PropertyId, select: Select },
    MultiSelect { id: Option<PropertyId> },
    Date { id: PropertyId },
    People { id: PropertyId },
    // XXX: in the spec it's `file`, but example has plural form.
    Files { id: PropertyId },
    Checkbox { id: PropertyId },
    Url { id: PropertyId },
    Email { id: PropertyId },
    PhoneNumber { id: PropertyId },
    Formula { id: PropertyId, formula: Formula },
    Relation { id: Option<PropertyId>, relation: Relation },
    Rollup { id: PropertyId, rollup: Rollup },
    CreatedTime { id: PropertyId },
    CreatedBy { id: PropertyId },
    LastEditedTime { id: PropertyId },
    LastEditedBy { id: PropertyId },
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize, Deref)]
#[serde(rename_all = "snake_case")]
pub struct Select {
    #[deref]
    pub options: Vec<SelectOption>,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SelectOption {
    pub id: String,
    pub name: String,
    pub color: Color,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Relation {
    pub database_id: Option<Uuid>,
    pub synced_property_name: Option<String>,
    pub synced_property_id: Option<PropertyId>,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct Formula {
    pub expression: String,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct Number {
    pub format: NumberFormat,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NumberFormat {
    Number,
    NumberWithCommas,
    Percent,
    Dollar,
    Euro,
    Pound,
    Yen,
    Ruble,
    Rupee,
    Won,
    Yuan,
    #[serde(other)]
    Unknown,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
pub struct Rollup {
    pub relation_property_name: PropertyName,
    pub relation_property_id: PropertyId,
    pub rollup_property_name: PropertyName,
    pub rollup_property_id: PropertyId,
    pub function: RollupFunction,
}

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollupFunction {
    Count,
    CountAll,
    CountValues,
    CountUniqueValues,
    CountEmpty,
    CountNotEmpty,
    PercentEmpty,
    PercentNotEmpty,
    Sum,
    Average,
    Median,
    Min,
    Max,
    Range,
    #[serde(other)]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rollup_function() {
        assert_eq!(
            serde_json::from_str::<RollupFunction>(r#""count""#).unwrap(),
            RollupFunction::Count
        );
        assert_eq!(
            serde_json::from_str::<RollupFunction>(r#""percent_not_empty""#).unwrap(),
            RollupFunction::PercentNotEmpty
        );
    }
}
