use super::ParseTypedFieldError;

pub fn expect_max_one_subfield(subfields: Vec<&str>) -> Result<Option<&str>, ParseTypedFieldError> {
    if subfields.len() == 0 {
        Ok(None)
    } else if subfields.len() == 1 {
        Ok(Some(subfields[0]))
    } else {
        Err("Expected only one such subfield".to_string())
    }
}

pub fn concat_subfields(subfields: Vec<&str>) -> Option<String> {
    if subfields.len() == 0 {
        None
    } else {
        Some(subfields.join(""))
    }
}
