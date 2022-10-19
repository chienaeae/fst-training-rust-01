use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    ids: Vec<Uuid>,
    names: Vec<String>,
}

impl Condition {
    #[inline]
    #[must_use]
    pub const fn empty() -> Self { Self { ids: Vec::new(), names: Vec::new() } }

    #[inline]
    #[must_use]
    pub fn with_id(id: Uuid) -> Self { Self { ids: vec![id], ..Self::empty() } }

    #[inline]
    #[must_use]
    pub fn with_ids(ids: &[Uuid]) -> Self { Self { ids: ids.to_vec(), ..Self::empty() } }

    #[inline]
    #[must_use]
    pub fn with_names(names: &[String]) -> Self { Self { names: names.to_vec(), ..Self::empty() } }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { *self == Self::empty() }

    #[inline]
    #[must_use]
    pub fn display(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        format!(" {self}")
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[")?;

        let separator = match self.ids.as_slice() {
            [] => false,
            [id] => {
                write!(f, "id={id}")?;
                true
            }
            ids => {
                let s = ids.iter().map(ToString::to_string).collect::<Vec<_>>().join(",");
                write!(f, "ids=[{s}]")?;
                true
            }
        };

        // names
        match self.names.as_slice() {
            [] => {}
            [name] => {
                if separator {
                    f.write_str(",")?;
                }
                write!(f, "name={name}")?;
            }
            names => {
                if separator {
                    f.write_str(",")?;
                }
                let s = names.join(",");
                write!(f, "names=[{s}]")?;
            }
        }

        f.write_str("]")
    }
}
