use cjtoolkit_structured_validator::types::description::{
    Description, DescriptionError, DescriptionRules,
};
use cjtoolkit_structured_validator::types::name::{Name, NameError, NameRules};

#[inline]
fn description_rules() -> DescriptionRules {
    DescriptionRules {
        is_mandatory: true,
        min_length: Some(5),
        max_length: Some(100),
    }
}

#[inline]
fn name_rules() -> NameRules {
    NameRules {
        is_mandatory: true,
        min_length: Some(5),
        max_length: Some(20),
    }
}

pub trait DescriptionBucketRulesExt {
    fn parse_bucket(s: Option<&str>) -> Result<Description, DescriptionError>;
}

impl DescriptionBucketRulesExt for Description {
    fn parse_bucket(s: Option<&str>) -> Result<Description, DescriptionError> {
        Self::parse_custom(s, description_rules())
    }
}

pub trait NameBucketRulesExt {
    fn parse_bucket(s: Option<&str>) -> Result<Name, NameError>;
}

impl NameBucketRulesExt for Name {
    fn parse_bucket(s: Option<&str>) -> Result<Name, NameError> {
        Self::parse_custom(s, name_rules())
    }
}
