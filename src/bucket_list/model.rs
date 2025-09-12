use crate::bucket_list::rules::{DescriptionBucketRulesExt, NameBucketRulesExt};
use crate::common::locale::{LocaleExtForResult, LocaleExtForStore};
use chrono::{DateTime, Utc};
use cjtoolkit_structured_validator::common::flag_error::FlagCounter;
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
            let mut flag = FlagCounter::new();

            let name = flag.check(Name::parse_bucket(Some(self.name.as_str())));
            let description =
                flag.check(Description::parse_bucket(Some(self.description.as_str())));

            if flag.is_flagged() {
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
            name: self.name.as_original_message(),
            description: self.description.as_original_message(),
        }
    }
}

impl Into<AddToBucketListValidationErrorResponse> for (AddToBucketListValidationError, &Locale) {
    fn into(self) -> AddToBucketListValidationErrorResponse {
        AddToBucketListValidationErrorResponse {
            name: self.0.name.as_translated_message(self.1),
            description: self.0.description.as_translated_message(self.1),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct AddToBucketListValidationErrorResponse {
    pub name: Arc<[String]>,
    pub description: Arc<[String]>,
}
