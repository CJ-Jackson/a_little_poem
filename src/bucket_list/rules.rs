use cjtoolkit_structured_validator::types::description::DescriptionRules;
use cjtoolkit_structured_validator::types::name::NameRules;

#[inline]
pub fn description_rules() -> DescriptionRules {
    DescriptionRules {
        is_mandatory: true,
        min_length: Some(5),
        max_length: Some(100),
    }
}

#[inline]
pub fn name_rules() -> NameRules {
    NameRules {
        is_mandatory: true,
        min_length: Some(5),
        max_length: Some(20),
    }
}
