use crate::common::locale::LocaleExtForStore;
use chrono::{DateTime, Utc};
use cjtoolkit_structured_validator::common::flag_error::flag_error;
use cjtoolkit_structured_validator::types::description::{Description, DescriptionError};
use cjtoolkit_structured_validator::types::name::{Name, NameError};
use poem::i18n::Locale;
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

pub struct AddToBucketListResult(
    pub Result<AddToBucketListValidated, AddToBucketListValidationError>,
);

impl Into<AddToBucketListResult> for &AddToBucketList {
    fn into(self) -> AddToBucketListResult {
        AddToBucketListResult((|| {
            let mut flag = false;

            use flag_error as fe;
            let name = fe(&mut flag, Name::parse(Some(self.name.clone().as_str())));
            let description = fe(
                &mut flag,
                Description::parse(Some(self.description.clone().as_str())),
            );

            if flag {
                return Err(AddToBucketListValidationError { name, description });
            }

            Ok(AddToBucketListValidated {
                name: name.unwrap_or_default(),
                description: description.unwrap_or_default(),
            })
        })())
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
            name: self
                .name
                .err()
                .map(|e| e.0.as_original_message())
                .unwrap_or_default(),
            description: self
                .description
                .err()
                .map(|e| e.0.as_original_message())
                .unwrap_or_default(),
        }
    }
}

impl Into<AddToBucketListValidationErrorResponse> for (AddToBucketListValidationError, &Locale) {
    fn into(self) -> AddToBucketListValidationErrorResponse {
        AddToBucketListValidationErrorResponse {
            name: self
                .0
                .name
                .err()
                .map(|e| e.0.as_translated_message(self.1))
                .unwrap_or_default(),
            description: self
                .0
                .description
                .err()
                .map(|e| e.0.as_translated_message(self.1))
                .unwrap_or_default(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct AddToBucketListValidationErrorResponse {
    pub name: Arc<[String]>,
    pub description: Arc<[String]>,
}
