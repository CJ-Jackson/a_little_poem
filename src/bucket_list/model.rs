use crate::bucket_list::validate::description::{Description, DescriptionError};
use crate::bucket_list::validate::name::{Name, NameError};
use crate::common::validation::error_flag;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub struct BucketListItem {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AddToBucketList {
    pub name: String,
    pub description: String,
}

impl AddToBucketList {
    pub fn to_validated(&self) -> Result<AddToBucketListValidated, AddToBucketListValidationError> {
        let mut flag = false;

        use error_flag as ef;
        let name = ef(&mut flag, Name::parse(self.name.clone()));
        let description = ef(&mut flag, Description::parse(self.description.clone()));

        if flag {
            return Err(AddToBucketListValidationError { name, description });
        }

        Ok(AddToBucketListValidated {
            name: name.unwrap_or_default(),
            description: description.unwrap_or_default(),
        })
    }
}

pub struct AddToBucketListValidated {
    pub name: Name,
    pub description: Description,
}

pub struct AddToBucketListValidationError {
    pub name: Result<Name, NameError>,
    pub description: Result<Description, DescriptionError>,
}

impl Into<AddToBucketListValidationErrorResponse> for AddToBucketListValidationError {
    fn into(self) -> AddToBucketListValidationErrorResponse {
        AddToBucketListValidationErrorResponse {
            name: self.name.err().map(|e| e.0).unwrap_or_default(),
            description: self.description.err().map(|e| e.0).unwrap_or_default(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct AddToBucketListValidationErrorResponse {
    pub name: Arc<[String]>,
    pub description: Arc<[String]>,
}
