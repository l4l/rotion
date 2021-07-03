use derive_more::Deref;
use serde::{Deserialize, Serialize};

mod block;
mod database;
mod page;
mod property;
mod property_values;
mod user;
mod values;

pub use block::*;
pub use database::*;
pub use page::*;
pub use property::*;
pub use property_values::*;
pub use user::*;
pub use values::*;

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Deref)]
#[serde(transparent)]
#[deref(forward)]
pub struct PropertyName(String);

#[cfg_attr(any(test, feature = "derive_debug"), derive(Debug))]
#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize, Deref)]
#[serde(transparent)]
#[deref(forward)]
pub struct PropertyId(String);
